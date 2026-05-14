//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1391/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1391<F: Float>(t21499: F, t33376: F, t32058: F, t415: F, t5975: F, t114618: F, t1411: F, t32197: F, t18958: F, t468: F, t110064: F, t110066: F, t110068: F, t110222: F, t110748: F, t110754: F, t110756: F, t113657: F, t114664: F, t32010: F, t32066: F, t33389: F, t33428: F, t9426: F) -> (F, F, F, F) {
    let t114674 = t33376 * t21499;
    let t114684 = t415 * t32058 * t5975;
    let t114687 = t1411 * t114618 * t32197;
    let t114694 = t415 * t468 * t18958;
    let t114698 = 0.69444444444444444446e-2 * t114664 * t32010 + 0.26805555555555555556e-2 * t114674 * t32010 + 0.26805555555555555556e-2 * t110222 * t33428 - 0.8041666666666666667e-2 * t9426 * t113657 - 0.24125000000000000001e-1 * t32066 * t33389 + 0.13265555555555555555e-1 * t114684 - 0.33163888888888888888e-2 * t114687 + 0.26805555555555555556e-2 * t110748 + 0.11054629629629629629e-2 * t110064 + 0.18424382716049382715e-2 * t110066 + 0.11054629629629629629e-2 * t110068 + 0.1621345679012345679e-1 * t114694 - 0.69444444444444444446e-2 * t110754 - 0.34722222222222222223e-2 * t110756;
    (t114684, t114687, t114694, t114698)
}
