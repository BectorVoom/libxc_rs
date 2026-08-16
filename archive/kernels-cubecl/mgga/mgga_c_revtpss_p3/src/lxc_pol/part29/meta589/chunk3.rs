//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1954/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1954<F: Float>(t530: F, t8107: F, t116: F, t28651: F, t13537: F, t13867: F, t2014: F, t22496: F, t2322: F, t2328: F, t25082: F, t25865: F, t26218: F, t26223: F, t26405: F, t26412: F, t27126: F, t28167: F, t28287: F, t28711: F, t28734: F, t33183: F, t35312: F, t3813: F, t4248: F, t4254: F, t4292: F, t49582: F, t5627: F, t651: F, t671: F, t7359: F, t7374: F, t7474: F, t75353: F, t7732: F, t7898: F, t7983: F, t8065: F, t9069: F, t98588: F) -> (F, F) {
    let t102015 = t530 * t8107;
    let t102019 = t28651 * t116;
    let t102058 = -F::cast_from(4.0_f64) * t4254 * t28711 - F::cast_from(2.0_f64) * t651 * t3813 * t7983 + F::cast_from(6.0_f64) * t2014 * t102015 * t25865 - F::cast_from(4.0_f64) * t102019 * t671 - F::cast_from(4.0_f64) * t2322 * t28734 - F::cast_from(4.0_f64) * t4254 * t28734 - F::cast_from(4.0_f64) * t651 * t7474 * t4292 - F::cast_from(2.0_f64) * t2328 * t8065 + F::cast_from(12.0_f64) * t28167 * t35312 * t5627 - F::cast_from(6.0_f64) * t25082 * t26405 * t75353 - F::cast_from(2.0_f64) * t7359 * t13537 - F::cast_from(4.0_f64) * t4248 * t26223 + F::cast_from(12.0_f64) * t28167 * t9069 * t13867 - F::cast_from(6.0_f64) * t25082 * t33183 * t22496 - F::cast_from(3.0_f64) * t25082 * t26405 * t49582 - F::cast_from(4.0_f64) * t27126 * t7374 - F::cast_from(2.0_f64) * t7732 * t26218 + F::cast_from(6.0_f64) * t7898 * t26412 + F::cast_from(4.0_f64) * t98588 * t28287;
    (t102019, t102058)
}
