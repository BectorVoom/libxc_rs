//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1102/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1102<F: Float>(t1262: F, t6326: F, t11020: F, t18653: F, t5302: F, t15227: F, t18648: F, t15231: F, t18657: F, t1662: F, t5336: F, t3515: F, t3530: F, t6837: F, t5329: F, t3500: F, t6770: F) -> (F, F, F, F, F, F, F) {
    let t20600 = t6326 * t1262;
    let t20601 = t11020 * t20600;
    let t20604 = t5302 * t18653;
    let t20607 = t15227 * t18648;
    let t20610 = t15231 * t18657;
    let t20613 = t1662 * t5336;
    let t20614 = t3515 * t20613;
    let t20617 = t3530 * t6837;
    let t20618 = t20617 * t1262;
    let t20619 = t5329 * t20618;
    let t20624 = t3500 * t6770;
    (t20601, t20604, t20607, t20610, t20614, t20619, t20624)
}
