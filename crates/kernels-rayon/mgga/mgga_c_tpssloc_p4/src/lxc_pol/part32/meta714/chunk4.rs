//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2247/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2247(t23053: f64, t5614: f64, t16859: f64, t6614: f64, t16673: f64, t6613: f64, t831: f64, t81736: f64, t81743: f64, t87206: f64, t87212: f64, t87213: f64, t98647: f64, t98651: f64, t98655: f64, t98659: f64, t98663: f64, t98668: f64, t98672: f64, t98674: f64, t98676: f64, t98678: f64) -> f64 {
    let t98680 = t23053 * t5614;
    let t98682 = t6614 * t16859;
    let t98684 = t16673 * t6613;
    let t98685 = t98684 * t831;
    let t98688 = 0.20186378047070195427e-3_f64 * t98647 - t87206 - t81736 + t81743 + 0.12111826828242117256e-2_f64 * t98651 - 0.40372756094140390854e-3_f64 * t98655 - 0.20186378047070195427e-3_f64 * t98659 + 0.12111826828242117256e-2_f64 * t98663 + 0.24223653656484234512e-2_f64 * t98668 + 0.24223653656484234512e-2_f64 * t98672 - 5.0_f64 / 192.0_f64 * t98674 + t98676 / 192.0_f64 - t98678 / 768.0_f64 - t98680 / 1536.0_f64 - t98682 / 1536.0_f64 - t98685 / 1536.0_f64 + t87212 + 0.33643963411783659045e-4_f64 * t87213;
    t98688
}
