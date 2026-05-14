//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1168/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1168<F: Float>(t3099: F, t422: F, t22522: F, t22572: F, t25704: F, t363: F, t401: F, t69: F, t8633: F, t100618: F, t100620: F, t100634: F, t11335: F, t1643: F, t1647: F, t1651: F, t1669: F, t1685: F, t1736: F, t1751: F, t18: F, t22541: F, t22558: F, t25759: F, t25802: F, t2983: F, t2992: F, t34433: F, t379: F, t411: F, t423: F, t5569: F, t5570: F, t73: F, t920: F, t925: F, t92531: F, t92546: F, t92559: F, t92571: F, t92575: F, t92872: F, t93048: F, t93157: F, t93169: F, t938: F) -> (F, F, F) {
    let t100645 = t422 * t3099;
    let t100667 = t22522 * t22572 * t25704;
    let t100669 = t401 * t363;
    let t100678 = t69 * t8633;
    let t100687 = -0.98910212891072794759e-5 * t100618 - 0.44540303667943584666e-4 * t5569 * t73 * t100620 - 0.45967398033333333333e0 * t1669 * t92872 * t5570 * t34433 * t11335 - 0.12768721675925925926e-1 * t22541 * t5570 * t423 * t920 * t1685 - 0.51074886703703703704e-1 * t22541 * t100634 * t423 * t18 * t401 + 0.12768721675925925926e-1 * t22522 * t5570 * t423 * t920 * t1751 + 0.25537443351851851852e-1 * t22522 * t5570 * t100645 * t379 + 0.12768721675925925926e-1 * t22522 * t5570 * t25759 * t1651 + 0.17024962234567901235e-1 * t22522 * t5570 * t1736 * t938 * t1643 + 0.23754828622903245156e-3 * t5569 * t411 * t25802 - t92531 - 0.25537443351851851852e-1 * t22522 * t5570 * t25759 * t1647 + 0.85124811172839506173e-2 * t100667 + 0.51074886703703703704e-1 * t22541 * t93169 * t2992 * t100669 - 0.60548059007656442388e-3 * t93157 * t93048 * t925 * t22558 - 0.3404992446913580247e-1 * t22541 * t100678 * t2983 * t100669 - 0.29693535778629056444e-3 * t92546 + 0.17024962234567901235e-1 * t92559 - 0.12768721675925925926e-1 * t92571 + 0.20182686335885480796e-3 * t92575;
    (t100669, t100678, t100687)
}
