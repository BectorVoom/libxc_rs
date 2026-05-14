//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1101/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1101<F: Float>(t1251: F, t20590: F, t1262: F, t6330: F, t3515: F, t11063: F, t11086: F, t11093: F, t11100: F, t20564: F, t20570: F, t20574: F, t20580: F, t20585: F, t3490: F, t3514: F, t6763: F, t6776: F) -> (F,) {
    let t20591 = t1251 * t20590;
    let t20593 = t6330 * t1262;
    let t20594 = t3515 * t20593;
    let t20598 = -t3514 * t20564 / 144.0 + t11086 * t6763 / 108.0 - t20570 / 864.0 + t3514 * t20574 / 144.0 - t11063 / 2592.0 + t3514 * t20580 / 288.0 - t3514 * t20585 / 576.0 - t3490 * t6776 / 36.0 + t20591 / 288.0 + t3514 * t20594 / 288.0 + t11093 + t11100 / 324.0;
    (t20598,)
}
