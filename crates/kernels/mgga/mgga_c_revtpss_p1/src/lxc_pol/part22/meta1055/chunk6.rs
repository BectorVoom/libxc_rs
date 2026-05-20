//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3736/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3736<F: Float>(t12987: F, t5390: F, t1250: F, t12910: F, t17347: F, t17448: F, t17635: F, t17753: F, t17754: F, t3584: F, t3720: F, t5402: F, t57100: F, t57726: F, t57735: F, t57743: F, t57746: F, t57749: F, t57770: F, t57773: F, t57780: F, t6688: F, t70890: F) -> F {
    let t70959 = t12987 * t5390;
    let t70978 = F::cast_from(0.85748036236139473944e-3_f64) * t12910 * t3720 * t6688 * t1250 * t3584 + F::cast_from(0.13719685797782315831e-1_f64) * t70959 * t17347 + F::cast_from(0.21437009059034868486e-3_f64) * t17753 * t3720 * t70890 * t17754 - t57726 / F::new(243.0) - F::cast_from(0.11433071498151929859e-2_f64) * t57735 - F::cast_from(0.57165357490759649296e-3_f64) * t57100 * t5402 - F::cast_from(0.57165357490759649296e-3_f64) * t17448 * t17635 - t57743 / F::new(108.0) - t57746 / F::new(216.0) - t57749 / F::new(72.0) - F::cast_from(0.3811023832717309953e-3_f64) * t57770 + F::cast_from(0.19055119163586549765e-3_f64) * t57773 - F::cast_from(0.57165357490759649296e-3_f64) * t57780;
    t70978
}
