//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1176/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1176<F: Float>(t1632: F, t45019: F, t100580: F, t100581: F, t100586: F, t100678: F, t100698: F, t100905: F, t100910: F, t100915: F, t100932: F, t11085: F, t1669: F, t1682: F, t1712: F, t22513: F, t22522: F, t22585: F, t22591: F, t22736: F, t22738: F, t22777: F, t22842: F, t25658: F, t25663: F, t25675: F, t25676: F, t25679: F, t25708: F, t25734: F, t2983: F, t38013: F, t45527: F, t45891: F, t5538: F, t5569: F, t58948: F, t6427: F, t73: F, t92489: F, t92699: F, t92782: F, t92809: F, t930: F, t93047: F, t93048: F, t93122: F, t93124: F) -> (F, F) {
    let t100954 = t45019 * t1632;
    let t100968 = -12.0 * t1669 * t92809 * t100698 + 0.51074886703703703704e-1 * t25708 * t100905 - t100910 - 0.60548059007656442388e-3 * t22513 * t100915 - 0.27568129967481981594e-3 * t5538 * t22777 * t25658 + 0.36061544906567819424e-6 * t45527 * t25663 + 0.87299078230359608381e-3 * t5538 * t92699 * t6427 + 0.60548059007656442388e-3 * t93047 * t93048 * t100580 * t100586 - 0.40365372671770961592e-3 * t22513 * t100932 + 0.3404992446913580247e-1 * t22522 * t100678 * t2983 * t100581 - 0.2370952259137005195e-1 * t22842 * t58948 - 0.2370952259137005195e-1 * t92489 * t25676 + 0.13336606457645654222e-1 * t38013 * t22591 * t25679 * t1712 + 0.75080154872671831175e-1 * t22842 * t25675 * t1682 - 0.29693535778629056444e-3 * t93122 * t22585 * t930 * t93124 + 0.53448364401532301599e-4 * t5569 * t73 * t100954 - 0.558117622714507008e-2 * t25734 * t11085 - 0.10417183504236821466e-4 * t22736 * t92782 * t6427 + 0.12255510004984495842e-5 * t22736 * t22738 * t25658 - 0.60102574844279699039e-6 * t45891 * t25663;
    (t100954, t100968)
}
