//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1028/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1028<F: Float>(t1092: F, t14604: F, t4793: F, t9429: F, t2861: F, t4815: F, t1773: F, t3219: F, t9546: F, t1021: F, t1767: F, t3228: F, t1022: F, t9589: F, t2855: F, t4818: F) -> (F, F, F, F, F, F, F, F) {
    let t14605 = t1092 * t14604;
    let t14607 = t9429 * t4793;
    let t14609 = t2861 * t4815;
    let t14611 = t1773 * t3219;
    let t14612 = t9546 * t14611;
    let t14613 = t1021 * t14612;
    let t14614 = t1092 * t14613;
    let t14616 = t1767 * t3228;
    let t14617 = t1022 * t14616;
    let t14618 = t9589 * t14617;
    let t14619 = t1092 * t14618;
    let t14622 = t2855 * t4818;
    (t14605, t14607, t14609, t14611, t14614, t14616, t14619, t14622)
}
