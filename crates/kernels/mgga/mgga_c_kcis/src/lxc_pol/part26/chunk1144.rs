//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1144/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1144<F: Float>(t1394: F, t5667: F, t98470: F, t20887: F, t4153: F, t7923: F, t1615: F, t6176: F, t7509: F, t94862: F, t27614: F, t7492: F, t12844: F, t27583: F, t29582: F, t18171: F, t29574: F) -> (F, F, F, F, F, F) {
    let t101868 = t1394 * t98470 * t5667;
    let t101871 = t4153 * t7923 * t20887;
    let t101875 = t6176 * t94862 * t7509 * t1615;
    let t101884 = t6176 * t27614 * t7492 * t1615;
    let t101892 = t27583 * t12844 * t29582;
    let t101894 = t18171 * t29574;
    (t101868, t101871, t101875, t101884, t101892, t101894)
}
