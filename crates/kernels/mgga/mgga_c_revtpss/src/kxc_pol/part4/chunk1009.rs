//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1009/1274 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk1009<F: Float>(t114: F, t13509: F, t655: F, t10201: F, t10202: F, t10204: F, t10206: F, t13448: F, t13451: F, t13453: F, t13455: F, t13459: F, t13462: F, t69: F, t10416: F, t1312: F, t13425: F, t13426: F, t13429: F, t13435: F, t13440: F, t1518: F, t2322: F, t2371: F, t4248: F, t4292: F, t5523: F, t670: F) -> (F, F) {
    let t115 = 1.0 < t114;
    let t13510 = t655 * t13509;
    let t13513 = -t10201 - 22.0 / 9.0 * t10202 - 2.0 / 3.0 * t10204 + t10206 / 3.0 - 11.0 / 9.0 * t13448 - t13451 + t13453 - 3.0 / 4.0 * t69 * t13455 + t69 * t13459 / 2.0 + t69 * t13462 / 4.0 - t69 * t13510 / 8.0;
    let t13514 = piecewise3(t115, 0.0, t13513);
    let t13517 = 2.0 * t10416 * t1518 + 2.0 * t1312 * t13514 + 4.0 * t13426 * t670 + 4.0 * t13435 * t1518 + 2.0 * t13440 * t1518 + 4.0 * t2322 * t4292 + 2.0 * t2371 * t4248 + 4.0 * t4292 * t5523 + t13425 + 2.0 * t13429;
    (t13514, t13517)
}
