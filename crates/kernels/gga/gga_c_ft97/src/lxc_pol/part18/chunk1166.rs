//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1166/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1166<F: Float>(t100586: F, t2992: F, t93048: F, t1593: F, t1632: F, t938: F, t11437: F, t5570: F, t8088: F, t5569: F, t6441: F, t93003: F, t22796: F, t25770: F, t93253: F, t100554: F, t100556: F, t100558: F, t100580: F, t100581: F, t11335: F, t22513: F, t22515: F, t22534: F, t22558: F, t22797: F, t22798: F, t25657: F, t25689: F, t25708: F, t25774: F, t34433: F, t411: F, t6442: F, t73: F, t92435: F, t92448: F, t92463: F, t92468: F, t92495: F, t925: F, t92533: F, t92873: F, t92999: F, t93011: F, t93117: F, t93136: F, t93143: F, t93168: F, t93169: F) -> (F, F, F, F) {
    let t100588 = t93048 * t2992 * t100586;
    let t100592 = t938 * t1593 * t1632;
    let t100601 = t5570 * t8088 * t11437;
    let t100610 = t5569 * t93003 * t6441;
    let t100613 = t22796 * t93253 * t25770;
    let t100615 = -0.39591381038172075258e-3 * t92463 * t25689 + 0.49489226297715094074e-4 * t100554 - 0.17816121467177433866e-3 * t93136 * t100556 * t100558 + 0.68099848938271604939e-1 * t92435 + 0.51074886703703703704e-1 * t93168 * t93169 * t925 * t11335 - 0.34724394379261436962e-6 * t92448 + 0.49489226297715094073e-4 * t92468 + 0.10560293360415908094e-4 * t22796 * t22797 * t25657 * t22798 - 0.60548059007656442388e-3 * t92495 + 0.79128170312858235807e-4 * t22534 * t411 * t25774 - 0.61601711269092797214e-4 * t22796 * t93143 * t25770 - 0.76612330055555555555e-1 * t92873 * t93169 * t100580 * t100581 + 0.60548059007656442388e-3 * t22513 * t100588 + 0.29673063867321838427e-4 * t92533 * t73 * t100592 + 0.36328835404593865432e-2 * t93117 * t22515 * t34433 * t22558 - 0.38306165027777777778e-1 * t25708 * t100601 + 0.44540303667943584666e-3 * t93011 * t6442 + 0.21775259570994641392e-2 * t5569 * t92999 * t6441 - 0.39591381038172075258e-3 * t100610 + 0.3520097786805302698e-5 * t100613;
    (t100588, t100592, t100601, t100615)
}
