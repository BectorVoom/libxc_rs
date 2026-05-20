//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1824/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1824<F: Float>(t543: F, t92063: F, t6843: F, t124: F, t1388: F, t1390: F, t1410: F, t1868: F, t22809: F, t3944: F, t4012: F, t46730: F, t48563: F, t74264: F, t74277: F, t74279: F, t74281: F, t74290: F, t800: F, t828: F, t85764: F, t85778: F, t85782: F, t85791: F, t85816: F, t91870: F, t91875: F, t91942: F) -> (F, F, F, F) {
    let t92064 = t92063 * t543;
    let t92069 = t6843 * t6843;
    let t92070 = t92069 * t543;
    let t92081 = -F::cast_from(0.80328230880474379775e-6_f64) * t48563 + F::new(5.0) / F::new(4.0) * t46730 * t800 * t124 * t91870 + F::new(3.0) / F::new(16.0) * t3944 * t800 * t124 * t91875 - F::cast_from(0.24009450146119052704e0_f64) * t85764 + F::cast_from(0.30492001685571196936e-2_f64) * t85778 + F::cast_from(0.40015750243531754508e-2_f64) * t85782 + F::cast_from(0.5421477899694558815e-3_f64) * t74264 - F::cast_from(0.21437009059034868486e-3_f64) * t1388 * t1390 * t828 * t91942 + F::cast_from(0.17149607247227894789e-1_f64) * t1410 * t4012 * t828 * t22809 * t1868 - F::cast_from(0.21437009059034868486e-3_f64) * t1388 * t1390 * t828 * t92064 - F::cast_from(0.64311027177104605458e-3_f64) * t1388 * t1390 * t828 * t92070 - F::new(7.0) / F::new(4.0) * t85791 + F::cast_from(0.30492001685571196935e-3_f64) * t85816 - F::cast_from(0.13605355082800796532e0_f64) * t74277 + F::cast_from(0.68026775414003982664e0_f64) * t74279 - F::cast_from(0.45732285992607719437e-3_f64) * t74281 - F::cast_from(0.16262400898971305032e-1_f64) * t74290;
    (t92064, t92069, t92070, t92081)
}
