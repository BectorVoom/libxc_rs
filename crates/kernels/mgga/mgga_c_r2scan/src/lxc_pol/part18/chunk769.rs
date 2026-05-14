//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 769/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk769<F: Float>(t2591: F, t8740: F, t7337: F, t8735: F, t5109: F, t495: F, t7321: F, t2551: F, t3090: F, t2573: F, t2122: F, t2133: F, t5101: F, t5108: F, t6132: F, t6139: F, t6293: F, t6583: F, t7235: F, t7237: F, t7259: F, t7263: F, t7298: F, t7312: F, t7317: F, t8737: F) -> (F, F, F, F, F, F, F) {
    let t8741 = t8740 * t2591;
    let t8742 = t7337 * t8741;
    let t8745 = t8735 * t2591;
    let t8746 = t5109 * t8745;
    let t8749 = t5109 * t8741;
    let t8752 = t8740 * t495;
    let t8753 = t7321 * t8752;
    let t8756 = t8740 * t2551;
    let t8757 = t7321 * t8756;
    let t8760 = t3090 * t495;
    let t8761 = t5109 * t8760;
    let t8764 = t8735 * t2573;
    let t8765 = t5109 * t8764;
    let t8768 = -0.25426783770825854452e1 * t7235 - 0.85366933852867742947e0 * t7237 - 0.12695991786046386925e-1 * t7259 - 0.38087975358139160777e-1 * t7263 + 0.16262400898971305031e-3 * t7298 + t7312 + t7317 - 0.16463622957338778997e-1 * t5101 + 0.86682217400542685632e-1 * t2133 * t8737 - 0.21951497276451705328e0 * t2122 * t8742 - 0.17336443480108537126e0 * t6132 * t8746 - 0.5200933044032561138e0 * t6139 * t8749 + 0.10975748638225852664e0 * t2122 * t8753 - 0.32927245914677557992e0 * t6293 * t8757 - 0.2600466522016280569e0 * t5108 * t8761 - 0.17336443480108537126e0 * t6583 * t8765;
    (t8741, t8745, t8752, t8756, t8760, t8764, t8768)
}
