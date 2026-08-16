//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1824/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1824(t543: f64, t92063: f64, t6843: f64, t124: f64, t1388: f64, t1390: f64, t1410: f64, t1868: f64, t22809: f64, t3944: f64, t4012: f64, t46730: f64, t48563: f64, t74264: f64, t74277: f64, t74279: f64, t74281: f64, t74290: f64, t800: f64, t828: f64, t85764: f64, t85778: f64, t85782: f64, t85791: f64, t85816: f64, t91870: f64, t91875: f64, t91942: f64) -> (f64, f64, f64, f64) {
    let t92064 = t92063 * t543;
    let t92069 = t6843 * t6843;
    let t92070 = t92069 * t543;
    let t92081 = -0.80328230880474379775e-6_f64 * t48563 + 5.0_f64 / 4.0_f64 * t46730 * t800 * t124 * t91870 + 3.0_f64 / 16.0_f64 * t3944 * t800 * t124 * t91875 - 0.24009450146119052704e0_f64 * t85764 + 0.30492001685571196936e-2_f64 * t85778 + 0.40015750243531754508e-2_f64 * t85782 + 0.5421477899694558815e-3_f64 * t74264 - 0.21437009059034868486e-3_f64 * t1388 * t1390 * t828 * t91942 + 0.17149607247227894789e-1_f64 * t1410 * t4012 * t828 * t22809 * t1868 - 0.21437009059034868486e-3_f64 * t1388 * t1390 * t828 * t92064 - 0.64311027177104605458e-3_f64 * t1388 * t1390 * t828 * t92070 - 7.0_f64 / 4.0_f64 * t85791 + 0.30492001685571196935e-3_f64 * t85816 - 0.13605355082800796532e0_f64 * t74277 + 0.68026775414003982664e0_f64 * t74279 - 0.45732285992607719437e-3_f64 * t74281 - 0.16262400898971305032e-1_f64 * t74290;
    (t92064, t92069, t92070, t92081)
}
