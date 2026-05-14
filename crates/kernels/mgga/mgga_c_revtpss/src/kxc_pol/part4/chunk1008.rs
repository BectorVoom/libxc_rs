//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1008/1274 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk1008<F: Float>(t22: F, t4273: F, t10241: F, t1509: F, t2358: F, t105: F, t2357: F, t2255: F, t661: F, t2362: F, t4279: F, t108: F, t580: F, t4283: F, t13472: F, t13475: F, t13476: F, t13479: F, t13482: F, t1505: F, t1507: F, t2344: F, t2359: F, t2363: F, t4270: F, t4274: F, t656: F, t97: F) -> (F,) {
    let t13485 = t4273 * t22;
    let t13493 = t10241 * t1509 * t2358;
    let t13496 = t105 * t2357;
    let t13497 = t2255 * t661;
    let t13500 = t4279 * t2362;
    let t13503 = t108 * t580;
    let t13506 = t4283 * t22;
    let t13509 = 200.0 / 27.0 * t2344 * t1505 - 100.0 / 27.0 * t656 * t4270 - 50.0 / 9.0 * t656 * t4274 - 10.0 / 27.0 * t97 * t13472 + 20.0 / 9.0 * t13475 * t13476 + 10.0 / 9.0 * t97 * t13479 + 5.0 / 3.0 * t97 * t13482 - 5.0 * t97 * t13485 - 50.0 / 27.0 * t1507 * t2359 - 25.0 / 9.0 * t1507 * t2363 - 10.0 / 27.0 * t105 * t13493 - 20.0 / 9.0 * t13496 * t13497 + 10.0 / 9.0 * t105 * t13500 - 5.0 / 3.0 * t105 * t13503 + 5.0 * t105 * t13506;
    (t13509,)
}
