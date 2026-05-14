//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1223/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1223<F: Float>(t4281: F, t7296: F, t2042: F, t6037: F, t1533: F, t1489: F, t6917: F, t4261: F, t6027: F, t17514: F, t2055: F, t17474: F, t5919: F, t1539: F, t7382: F, t4293: F) -> (F, F, F, F, F, F, F) {
    let t22659 = t4281 * t7296;
    let t22661 = t2042 * t6037;
    let t22662 = t1533 * t22661;
    let t22664 = t6917 * t1489;
    let t22665 = t4261 * t22664;
    let t22666 = t6027 * t22665;
    let t22668 = t17514 * t2055;
    let t22670 = t17474 * t5919;
    let t22672 = t7382 * t1539;
    let t22674 = t4293 * t22664;
    (t22659, t22662, t22666, t22668, t22670, t22672, t22674)
}
