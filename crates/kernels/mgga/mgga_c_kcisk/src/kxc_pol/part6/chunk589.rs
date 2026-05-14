//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 589/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk589<F: Float>(t1725: F, t8697: F, t2408: F, t4864: F, t4868: F, t7076: F, t8684: F, t8687: F, t8690: F, t1707: F, t4881: F, t1714: F, t1248: F, t4893: F, t8510: F, t1720: F, t8514: F) -> (F, F, F, F, F, F, F, F, F) {
    let t8698 = t8697 * t1725;
    let t8701 = t2408 * t2408;
    let t8702 = t4864 * t8701;
    let t8708 = t4868 + 2.0 / 9.0 * t7076 - 2.0 / 9.0 * t8684 + 2.0 / 3.0 * t8687 - t8690 / 3.0;
    let t8709 = t1707 * t8708;
    let t8715 = t4881 * t8701;
    let t8717 = t1714 * t8708;
    let t8721 = t1248 * t4893 * t8510;
    let t8724 = t1248 * t1720 * t8514;
    (t8698, t8701, t8702, t8708, t8709, t8715, t8717, t8721, t8724)
}
