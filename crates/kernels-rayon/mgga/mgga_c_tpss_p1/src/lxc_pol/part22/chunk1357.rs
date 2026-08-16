//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1357/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1357(t1692: f64, t1812: f64, t18239: f64, t18728: f64, t18807: f64, t18812: f64, t20025: f64, t20054: f64, t20417: f64, t20526: f64, t2439: f64, t3552: f64, t35530: f64, t5849: f64, t5853: f64, t6214: f64, t62820: f64, t6354: f64, t6380: f64, t64870: f64, t64876: f64, t64905: f64, t64914: f64, t64917: f64, t64941: f64, t64946: f64, t64976: f64, t64986: f64, t64997: f64) -> f64 {
    let t66833 = 3.0_f64 * t2439 * t5849 * t20025 + t1692 * t18812 * t64876 + 3.0_f64 * t3552 * t6354 * t18239 - t1692 * t62820 * t6214 / 2.0_f64 + 2.0_f64 * t20526 * t64941 - 3.0_f64 * t18728 * t64976 + 6.0_f64 * t20417 * t64914 + 6.0_f64 * t20417 * t64997 + 3.0_f64 * t20417 * t64870 + 3.0_f64 / 2.0_f64 * t2439 * t1812 * t64905 - 3.0_f64 * t18728 * t64986 - t1692 * t5853 * t64946 / 2.0_f64 + 3.0_f64 * t3552 * t1812 * t64917 - t1692 * t18807 * t20054 + 3.0_f64 * t35530 * t6380;
    t66833
}
