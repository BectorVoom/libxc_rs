//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1358/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1358(t1692: f64, t1812: f64, t18250: f64, t18265: f64, t18728: f64, t18803: f64, t18807: f64, t20021: f64, t20041: f64, t20065: f64, t20417: f64, t20526: f64, t2439: f64, t5849: f64, t5853: f64, t6207: f64, t62610: f64, t6354: f64, t64880: f64, t64928: f64, t64950: f64, t64954: f64, t64958: f64, t64969: f64, t64979: f64, t64992: f64, t65002: f64, t66299: f64) -> f64 {
    let t66870 = -t1692 * t18807 * t20065 + 3.0_f64 / 2.0_f64 * t2439 * t1812 * t64950 - 3.0_f64 * t20526 * t64880 + 3.0_f64 * t2439 * t5849 * t20021 - 3.0_f64 * t18728 * t65002 + 3.0_f64 / 2.0_f64 * t2439 * t1812 * t64992 + 3.0_f64 / 2.0_f64 * t2439 * t18803 * t6207 - 3.0_f64 / 2.0_f64 * t18728 * t64969 + 2.0_f64 * t20526 * t64958 - 6.0_f64 * t20417 * t64928 + 3.0_f64 * t2439 * t6354 * t18250 - 3.0_f64 * t62610 * t20041 - 3.0_f64 / 2.0_f64 * t18728 * t64979 - t1692 * t5853 * t64954 + t1692 * t66299 * t18265;
    t66870
}
