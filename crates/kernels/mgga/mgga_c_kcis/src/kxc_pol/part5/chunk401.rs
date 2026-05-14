//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 401/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk401<F: Float>(t1489: F, t584: F, t583: F, t1546: F, t1497: F, t555: F, t578: F, t1530: F, t1536: F, t1540: F, t1544: F) -> (F, F, F, F, F, F, F) {
    let t1547 = t584 * t1489;
    let t1548 = t583 * t1547;
    let t1549 = t1546 * t1548;
    let t1551 = t555 * t1497;
    let t1552 = t583 * t1551;
    let t1553 = t578 * t1552;
    let t1555 = t1530 / 16.0 - t1536 / 16.0 + t1540 / 24.0 - t1544 / 256.0 + t1549 / 256.0 - t1553 / 192.0;
    (t1547, t1548, t1549, t1551, t1552, t1553, t1555)
}
