//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 712/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk712(t13816: f64, t35620: f64, t13809: f64, t7491: f64, t34709: f64, t34786: f64, t14063: f64, t3151: f64, t7472: f64, t118: f64, t1986: f64, t495: f64, t665: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t69936 = t35620 * t13816;
    let t69938 = t7491 * t13809;
    let t69940 = t34709 * t13816;
    let t69942 = t34786 * t13816;
    let t69953 = t7472 * t14063 * t3151;
    let t69971 = t1986 * t118 * t665 * t495;
    (t69936, t69938, t69940, t69942, t69953, t69971)
}
