//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1025/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1025<F: Float>(t1697: F, t2835: F, t1141: F, t5034: F, t1778: F, t3329: F, t13105: F, t381: F, t1795: F, t3225: F, t3436: F, t5025: F) -> (F, F, F, F, F, F) {
    let t14654 = t1697 * t2835;
    let t14665 = t5034 * t1141;
    let t14668 = t1778 * t3329;
    let t14721 = t13105 * t381;
    let t14781 = t1795 * t3225;
    let t14785 = t5025 * t3436;
    (t14654, t14665, t14668, t14721, t14781, t14785)
}
