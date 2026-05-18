//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 429/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk429<F: Float>(t1503: F, t1556: F, t1625: F, t1629: F, t1636: F, t187: F, t633: F, t449: F, t828: F, t89: F) -> (F, F, F) {
    let t1640 = t1503 - t1556 + t187 * (t1625 * t633 - t1629 * t1636 - t1503 + t1556);
    let t1641 = t449 * t1640;
    let t1646 = -t89 - t828;
    (t1640, t1641, t1646)
}
