//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1191/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1191(t225: f64, t24141: f64, t2085: f64, t3850: f64, t12178: f64, t12267: f64, t1336: f64, t1352: f64, t24116: f64, t24128: f64, t24131: f64, t3773: f64, t3777: f64, t3856: f64, t5250: f64, t5334: f64, t5344: f64, t7208: f64, t7209: f64, t7211: f64, t81016: f64, t81019: f64, t81022: f64, t81031: f64, t81037: f64, t81039: f64, t81041: f64, t81043: f64, t81047: f64, t81050: f64) -> (f64, f64) {
    let t84433 = t24141 * t225;
    let t84441 = t2085 * t3850;
    let t84471 = -3.0_f64 * t5344 * t84441 * t1352 + 6.0_f64 * t5334 * t84441 * t5250 + 0.9869604401089358619e-1_f64 * t81016 + 0.9869604401089358619e-1_f64 * t81019 - 0.49348022005446793095e-1_f64 * t81022 - 0.9869604401089358619e-1_f64 * t81031 + 3.0_f64 * t3773 * t7211 - t1336 * t7208 * t12178 - 3.0_f64 * t12267 * t7209 - 3.0_f64 * t3777 * t24131 + 6.0_f64 * t3777 * t24128 - 3.0_f64 * t1336 * t24116 * t3856 - 0.11514538467937585055e0_f64 * t81037 + 0.38381794893125283518e0_f64 * t81039 + 0.11514538467937585055e0_f64 * t81041 - 0.69087230807625510332e0_f64 * t81043 - 0.15626873635058151147e0_f64 * t81047 + 0.49348022005446793095e-1_f64 * t81050;
    (t84433, t84471)
}
