//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1285/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1285(t1936: f64, t60551: f64, t13440: f64, t7002: f64, t25832: f64, t5523: f64, t112: f64, t843: f64, t239: f64, t655: f64, t665: f64, t2339: f64, t624: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t94968 = 2.0_f64 * t60551 * t1936;
    let t94970 = 6.0_f64 * t13440 * t7002;
    let t94972 = 6.0_f64 * t5523 * t25832;
    let t94973 = t843 * t112;
    let t94974 = 154.0_f64 / 27.0_f64 * t94973;
    let t94975 = t239 * t655;
    let t94976 = t94975 * t665;
    let t94978 = t624 * t2339;
    (t94968, t94970, t94972, t94974, t94976, t94978)
}
