//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1448/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1448<F: Float>(t2259: F, t7217: F, t1554: F, t1556: F, t1562: F, t19753: F, t19768: F, t23041: F, t24000: F, t24046: F, t24084: F, t24131: F, t24179: F, t24232: F, t24283: F, t24331: F, t24428: F, t24469: F, t24506: F, t24526: F, t24570: F, t24621: F, t24657: F, t24700: F, t24727: F, t24766: F, t24802: F, t24829: F, t24869: F, t24899: F, t24939: F, t24990: F, t25088: F, t25128: F, t25176: F, t25222: F, t25260: F, t2531: F, t25311: F, t2533: F, t2534: F, t25355: F, t2538: F, t25387: F, t25414: F, t25446: F, t25465: F, t25492: F, t25527: F, t25551: F, t25590: F, t25636: F, t25668: F, t25706: F, t25733: F, t25761: F, t25802: F, t25845: F, t25893: F, t25934: F, t25950: F, t26001: F, t26044: F, t26093: F, t26126: F, t26169: F, t26215: F, t26246: F, t26292: F, t26333: F, t27014: F, t27052: F, t27083: F, t27107: F, t27181: F, t27211: F, t27247: F, t27285: F, t285: F, t3270: F, t494: F, t495: F, t496: F, t499: F, t5066: F, t5068: F, t5073: F, t5078: F, t5081: F, t5088: F, t6592: F, t7194: F, t7195: F, t7202: F, t7218: F, t8707: F, t920: F, t921: F, t983: F) -> (F,) {
    let t27311 = t7217 * t2259;
    let t27324 = -15.0 / 16.0 * t1554 * t7218 + t5066 * t2538 / 4.0 - 15.0 / 8.0 * t495 * t24000 + 3.0 / 4.0 * t7195 * t1556 + t499 * (t27052 + t26246 + t27083 + t27181 + t26292 + t26044 + t26126 + t26215 + t25260 + t24829 + t24766 + t24283 + t25668 + t25176 + t24802 + t27247 + t24899 + t25527 + t26093 + t24657 + t27107 + t25465 + t24428 + t25845 + t25636 + t25590 + t25934 + t26001 + t24939 + t24526 + t26169 + t24727 + t27211 + t24469 + t27285 + t25387 + t26333 + t25222 + t25761 + t25128 + t24179 + t25311 + t25551 + t24990 + t24700 + t25706 + t24131 + t25446 + t24084 + t25733 + t25802 + t24331 + t24046 + t24621 + t25893 + t25414 + t24232 + t25355 + t24869 + t27014 + t24506 + t24570 + t25950 + t25492) / 4.0 + 45.0 / 64.0 * t921 * t19753 + 3.0 / 4.0 * t5068 * t8707 - 585.0 / 256.0 * t23041 * t983 * t5088 - 15.0 / 16.0 * t2531 * t5078 - 5.0 / 16.0 * t1562 * t983 * t6592 - 15.0 / 16.0 * t921 * t19768 + 3.0 * t7194 * t494 * t2534 - 15.0 / 16.0 * t495 * t27311 + t25088 * t285 + 3.0 / 4.0 * t2533 * t3270 * t2259 + t920 * t5073 * t2534 + 3.0 / 4.0 * t2531 * t5081 + 3.0 * t7202 * t496;
    (t27324,)
}
