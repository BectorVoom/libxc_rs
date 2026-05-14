//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 847/884 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk847<F: Float>(t33565: F, t7372: F, t33294: F, t9810: F, t43107: F, t701: F, t6066: F, t7630: F, t41136: F, t41139: F, t41143: F, t43640: F, t43642: F, t43645: F, t43647: F, t43648: F, t43650: F, t43653: F, t43658: F, t43661: F, t43664: F, t43666: F, t43670: F, t43674: F, t43677: F) -> (F, F) {
    let t43679 = t33565 * t7372;
    let t43680 = 0.29792074959875355558e-1 * t43679;
    let t43681 = t33294 * t9810;
    let t43682 = 0.3575048995185042667e0 * t43681;
    let t43683 = t43107 * t701;
    let t43686 = 0.71500979903700853338e0 * t7630 * t6066 * t43683;
    let t43687 = t43640 + 0.23005755572352449806e2 * t43642 + t43645 + t43647 - t43648 + 0.19171462976960374838e1 * t43650 + t43653 - 0.1533717038156829987e1 * t41136 - 0.76685851907841499352e0 * t41139 + 0.76685851907841499352e0 * t41143 + t43658 + t43661 + t43664 - 0.15889106645266856298e0 * t43666 - t43670 - t43674 - 0.79445533226334281487e-1 * t43677 - t43680 + t43682 - t43686;
    (t43683, t43687)
}
