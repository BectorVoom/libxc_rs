//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1126/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1126<F: Float>(t16150: F, t5570: F, t8101: F, t4417: F, t53: F, t1737: F, t22515: F, t8120: F, t428: F, t4474: F, t3099: F, t938: F, t383: F, t930: F, t100540: F, t100541: F, t100580: F, t115385: F, t1570: F, t22513: F, t22522: F, t22585: F, t22603: F, t22761: F, t22777: F, t25698: F, t25703: F, t25708: F, t29502: F, t3188: F, t401: F, t5540: F, t5579: F, t72: F, t92348: F, t925: F, t92559: F, t92575: F, t92579: F, t92873: F, t93122: F, t93168: F, t93169: F) -> (F, F, F, F, F, F, F, F) {
    let t115583 = t5570 * t8101 * t16150;
    let t115586 = t4417 * t53;
    let t115588 = t22515 * t1737 * t115586;
    let t115592 = t5570 * t8120 * t16150;
    let t115603 = t4474 * t428;
    let t115608 = t938 * t3099;
    let t115617 = t930 * t938 * t383;
    let t115636 = -0.29693535778629056444e-3 * t93122 * t22585 * t930 * t925 * t428 + 0.51074886703703703704e-1 * t25708 * t115583 - 0.20182686335885480796e-3 * t22513 * t115588 - 0.19862455940329218107e-1 * t25708 * t115592 + 0.51690243689028715488e-5 * t22603 * t5540 * t115385 - 0.22983699016666666666e0 * t92348 * t5579 * t72 * t4474 * t401 + 0.45967398033333333332e0 * t92579 * t5579 * t72 * t115603 - 0.22983699016666666666e0 * t22761 * t5579 * t72 * t115608 + 0.27568129967481981594e-3 * t22603 * t22777 * t29502 + 0.10338048737805743098e-4 * t100540 * t100541 * t115617 + 0.85124811172839506174e-2 * t92559 + 0.10091343167942740398e-3 * t92575 + 0.51074886703703703704e-1 * t93168 * t93169 * t100580 * t25698 - 0.76612330055555555556e-1 * t92873 * t93169 * t100580 * t25703 - 0.51074886703703703704e-1 * t22522 * t93169 * t938 * t1570 * t3188;
    (t115583, t115586, t115588, t115592, t115603, t115608, t115617, t115636)
}
