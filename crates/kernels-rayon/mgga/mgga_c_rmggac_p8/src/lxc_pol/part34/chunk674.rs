//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 674/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk674(t326: f64, t7417: f64, t1179: f64, t14024: f64, t3899: f64, t14113: f64, t14118: f64, t2144: f64, t2529: f64, t1971: f64, t3121: f64, t14114: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t68471 = t7417 * t326;
    let t68489 = t1179 * t3899 * t14024;
    let t68490 = t14113 * t68489;
    let t68491 = t68490 * t14118;
    let t68498 = t2144 * t2529;
    let t68499 = t1971 * t68498;
    let t68502 = t14024 * t3121;
    let t68503 = t14114 * t68502;
    (t68471, t68489, t68490, t68491, t68499, t68503)
}
