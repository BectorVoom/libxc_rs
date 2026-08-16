//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2123/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2123(t1315: f64, t1453: f64, t1847: f64, t1911: f64, t21814: f64, t21882: f64, t21891: f64, t22506: f64, t22525: f64, t2322: f64, t4248: f64, t4254: f64, t4293: f64, t4297: f64, t508: f64, t511: f64, t5528: f64, t569: f64, t5787: f64, t5887: f64, t649: f64, t651: f64, t6765: f64, t6773: f64, t6934: f64, t7732: f64) -> f64 {
    let t22531 = t1315 * t6934 + t1453 * t6773 + 2.0_f64 * t1847 * t5787 + 2.0_f64 * t1911 * t5528 - t21814 * t508 - 2.0_f64 * t21882 * t651 - 4.0_f64 * t21891 * t651 + t22506 * t511 + t22525 * t569 - 4.0_f64 * t2322 * t5887 - 4.0_f64 * t4248 * t4293 - 4.0_f64 * t4248 * t4297 - 4.0_f64 * t4254 * t5887 - 4.0_f64 * t4293 * t7732 - t649 * t6765;
    t22531
}
