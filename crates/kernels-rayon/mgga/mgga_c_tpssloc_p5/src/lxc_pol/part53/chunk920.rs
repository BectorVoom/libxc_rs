//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 920/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk920(t1510: f64, t31993: f64, t235: f64, t33947: f64, t1499: f64, t226: f64, t31987: f64, t31989: f64, t33377: f64, t33381: f64, t33385: f64, t812: f64, t8738: f64) -> (f64, f64, f64) {
    let t33969 = t31993 * t1510;
    let t33971 = t235 * t33947;
    let t33973 = -t31987 - 0.6579736267392905746e-1_f64 * t33377 - t31989 - 0.3289868133696452873e-1_f64 * t33381 + 0.3289868133696452873e-1_f64 * t33385 + t1499 * t8738 - t812 * t33969 + t226 * t33971;
    (t33969, t33971, t33973)
}
