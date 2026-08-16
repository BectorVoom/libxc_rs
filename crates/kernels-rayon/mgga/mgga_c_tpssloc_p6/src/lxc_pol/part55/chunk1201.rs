//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1201/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1201(t6562: f64, t8335: f64, t86893: f64, t214: f64, t7510: f64, t1880: f64, t6572: f64, t10109: f64, t112908: f64, t112936: f64, t112942: f64, t118886: f64, t118892: f64, t118894: f64, t118895: f64, t118901: f64, t13065: f64, t1528: f64, t23278: f64, t25168: f64, t259: f64, t2713: f64, t30741: f64, t32853: f64, t4142: f64, t4272: f64, t4301: f64, t7538: f64, t8347: f64, t8362: f64, t8363: f64, t866: f64) -> (f64, f64) {
    let t118903 = t6562 * t86893 * t8335;
    let t118904 = 0.82246703342411321825e-2_f64 * t118903;
    let t118910 = t214 * t7510;
    let t118913 = 0.16449340668482264365e-1_f64 * t1880 * t118910 * t6572;
    let t118914 = -6.0_f64 * t10109 * t25168 * t4272 * t8362 + t259 * t4142 * t8347 - t112908 * t1528 - t118895 * t866 - t13065 * t8363 - 2.0_f64 * t23278 * t7538 - t2713 * t32853 - t30741 * t4301 + t112936 - t112942 + t118886 + t118892 - t118894 - t118901 + t118904 - t118913;
    (t118910, t118914)
}
