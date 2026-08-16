//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2241/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2241<F: Float>(t23053: F, t5614: F, t16859: F, t6614: F, t16673: F, t6613: F, t831: F, t81736: F, t81743: F, t87206: F, t87212: F, t87213: F, t98647: F, t98651: F, t98655: F, t98659: F, t98663: F, t98668: F, t98672: F, t98674: F, t98676: F, t98678: F) -> F {
    let t98680 = t23053 * t5614;
    let t98682 = t6614 * t16859;
    let t98684 = t16673 * t6613;
    let t98685 = t98684 * t831;
    let t98688 = F::cast_from(0.20186378047070195427e-3_f64) * t98647 - t87206 - t81736 + t81743 + F::cast_from(0.12111826828242117256e-2_f64) * t98651 - F::cast_from(0.40372756094140390854e-3_f64) * t98655 - F::cast_from(0.20186378047070195427e-3_f64) * t98659 + F::cast_from(0.12111826828242117256e-2_f64) * t98663 + F::cast_from(0.24223653656484234512e-2_f64) * t98668 + F::cast_from(0.24223653656484234512e-2_f64) * t98672 - F::cast_from(5.0_f64) / F::cast_from(192.0_f64) * t98674 + t98676 / F::cast_from(192.0_f64) - t98678 / F::cast_from(768.0_f64) - t98680 / F::cast_from(1536.0_f64) - t98682 / F::cast_from(1536.0_f64) - t98685 / F::cast_from(1536.0_f64) + t87212 + F::cast_from(0.33643963411783659045e-4_f64) * t87213;
    t98688
}
