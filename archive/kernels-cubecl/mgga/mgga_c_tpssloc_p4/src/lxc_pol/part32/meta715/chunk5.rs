//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2258/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2258<F: Float>(t23127: F, t5628: F, t16985: F, t6621: F, t1516: F, t87321: F, t25068: F, t4261: F, t5624: F, t23133: F, t87340: F, t16673: F, t6620: F) -> (F, F, F, F, F, F, F, F) {
    let t98818 = t23127 * t5628;
    let t98820 = t6621 * t16985;
    let t98822 = t87321 * t1516;
    let t98824 = t25068 * t4261;
    let t98826 = t23127 * t5624;
    let t98828 = t23133 * t5624;
    let t98830 = t87340 * t1516;
    let t98832 = t16673 * t6620;
    (t98818, t98820, t98822, t98824, t98826, t98828, t98830, t98832)
}
