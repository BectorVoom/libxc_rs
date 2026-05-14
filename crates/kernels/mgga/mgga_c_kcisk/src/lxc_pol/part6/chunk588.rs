//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 588/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk588<F: Float>(t1801: F, t8672: F, t5062: F, t1869: F, t2527: F, t6697: F, t1873: F, t1224: F, t4840: F, t8510: F, t1697: F, t8514: F, t8518: F, t4835: F, t7076: F, t2417: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t8673 = t1801 * t8672;
    let t8674 = t5062 * t8673;
    let t8675 = t1869 * t8674;
    let t8677 = t6697 * t2527;
    let t8678 = t1873 * t8677;
    let t8679 = t1869 * t8678;
    let t8684 = t1224 * t4840 * t8510;
    let t8687 = t1224 * t1697 * t8514;
    let t8690 = t1224 * t1697 * t8518;
    let t8692 = t4835 + 0.11872222222222222222e-1 * t7076 - 0.11872222222222222222e-1 * t8684 + 0.35616666666666666666e-1 * t8687 - 0.17808333333333333333e-1 * t8690;
    let t8697 = t2417 * t2417;
    (t8673, t8674, t8675, t8677, t8678, t8679, t8684, t8687, t8690, t8692, t8697)
}
