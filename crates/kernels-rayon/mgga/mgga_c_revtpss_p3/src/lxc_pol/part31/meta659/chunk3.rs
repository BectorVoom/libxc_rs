//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2233/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2233(t28043: f64, t4248: f64, t651: f64, t6765: f64, t7002: f64, t108716: f64, t108718: f64, t108721: f64, t108723: f64, t108725: f64, t108727: f64, t109006: f64, t109012: f64, t109014: f64, t1310: f64, t2007: f64, t21814: f64, t21891: f64, t25805: f64, t28025: f64, t28030: f64, t28050: f64, t29569: f64, t4297: f64, t508: f64, t5877: f64, t5887: f64, t6985: f64, t7221: f64, t7732: f64) -> f64 {
    let t109024 = 4.0_f64 * t4248 * t28043;
    let t109029 = 2.0_f64 * t651 * t6765 * t7002;
    let t109030 = -t109006 * t508 - t1310 * t29569 - t2007 * t21814 - 4.0_f64 * t21891 * t6985 - 4.0_f64 * t25805 * t5887 - 4.0_f64 * t28025 * t5887 - 4.0_f64 * t28030 * t4297 - 4.0_f64 * t28050 * t7732 - t5877 * t7221 - t108716 - t108718 - t108721 - t108723 - t108725 - t108727 - t109012 + t109014 - t109024 - t109029;
    t109030
}
