//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 503/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk503(t41: f64, t4594: f64, t4597: f64, t702: f64, t1849: f64, t5060: f64, t732: f64, t1934: f64, t718: f64, t642: f64, t5061: f64, t740: f64, sigma2: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5248 = t41 * t4594;
    let t5249 = t702 * t4597;
    let t5259 = t702 * t1849;
    let t5283 = t732 * t5060;
    let t5284 = t5283 * sigma2;
    let t5289 = t1934 * t718;
    let t5290 = t41 * t642;
    let t5315 = t5061 * t740;
    (t5248, t5249, t5259, t5283, t5284, t5289, t5290, t5315)
}
