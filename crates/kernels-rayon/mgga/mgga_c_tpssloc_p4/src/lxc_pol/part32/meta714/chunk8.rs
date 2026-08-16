//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2251/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2251(t25115: f64, t7496: f64, t87451: f64, t23133: f64, t5628: f64, t23041: f64, t5614: f64, t1512: f64, t87261: f64, t81850: f64, t81853: f64, t87292: f64, t87293: f64, t87301: f64, t87306: f64, t92633: f64, t98715: f64, t98717: f64, t98719: f64, t98721: f64, t98723: f64, t98725: f64, t98728: f64) -> f64 {
    let t98731 = t87451 * t7496 * t25115;
    let t98733 = t23133 * t5628;
    let t98736 = t23041 * t5614;
    let t98738 = t87261 * t1512;
    let t98740 = -t81850 - t81853 + t87292 + 0.16956557559538964159e-1_f64 * t87293 - t87301 + 5.0_f64 / 192.0_f64 * t98715 - 5.0_f64 / 64.0_f64 * t98717 + 5.0_f64 / 192.0_f64 * t98719 + 5.0_f64 / 384.0_f64 * t98721 - t98723 / 1536.0_f64 + 0.14130464632949136799e-2_f64 * t98725 - 0.48447307312968469024e-2_f64 * t98728 + 0.24223653656484234512e-2_f64 * t98731 + 7.0_f64 / 576.0_f64 * t98733 - t92633 - 0.13565246047631171327e0_f64 * t87306 + 7.0_f64 / 2304.0_f64 * t98736 + 7.0_f64 / 1152.0_f64 * t98738;
    t98740
}
