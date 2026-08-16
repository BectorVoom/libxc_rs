//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1954/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1954(t530: f64, t8107: f64, t116: f64, t28651: f64, t13537: f64, t13867: f64, t2014: f64, t22496: f64, t2322: f64, t2328: f64, t25082: f64, t25865: f64, t26218: f64, t26223: f64, t26405: f64, t26412: f64, t27126: f64, t28167: f64, t28287: f64, t28711: f64, t28734: f64, t33183: f64, t35312: f64, t3813: f64, t4248: f64, t4254: f64, t4292: f64, t49582: f64, t5627: f64, t651: f64, t671: f64, t7359: f64, t7374: f64, t7474: f64, t75353: f64, t7732: f64, t7898: f64, t7983: f64, t8065: f64, t9069: f64, t98588: f64) -> (f64, f64) {
    let t102015 = t530 * t8107;
    let t102019 = t28651 * t116;
    let t102058 = -4.0_f64 * t4254 * t28711 - 2.0_f64 * t651 * t3813 * t7983 + 6.0_f64 * t2014 * t102015 * t25865 - 4.0_f64 * t102019 * t671 - 4.0_f64 * t2322 * t28734 - 4.0_f64 * t4254 * t28734 - 4.0_f64 * t651 * t7474 * t4292 - 2.0_f64 * t2328 * t8065 + 12.0_f64 * t28167 * t35312 * t5627 - 6.0_f64 * t25082 * t26405 * t75353 - 2.0_f64 * t7359 * t13537 - 4.0_f64 * t4248 * t26223 + 12.0_f64 * t28167 * t9069 * t13867 - 6.0_f64 * t25082 * t33183 * t22496 - 3.0_f64 * t25082 * t26405 * t49582 - 4.0_f64 * t27126 * t7374 - 2.0_f64 * t7732 * t26218 + 6.0_f64 * t7898 * t26412 + 4.0_f64 * t98588 * t28287;
    (t102019, t102058)
}
