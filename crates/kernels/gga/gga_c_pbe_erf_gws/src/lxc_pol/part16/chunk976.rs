//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 976/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk976<F: Float>(t1114: F, t6159: F, t6154: F, t2362: F, t2397: F, t3083: F, t2373: F, t3066: F, t4390: F, t4425: F, t4430: F, t4443: F, t4454: F, t4467: F, t4469: F, t4484: F, t6164: F, t833: F, t8629: F, t8634: F, t8641: F, t8643: F, t8646: F, t8649: F, t8654: F) -> F {
    let t8659 = t1114 * t6159;
    let t8662 = t1114 * t6154;
    let t8664 = F::new(7.0) / F::new(144.0) * t8662 * t2362;
    let t8666 = F::new(7.0) / F::new(144.0) * t3083 * t2397;
    let t8667 = t8629 * t4390 / F::new(24.0) + t8629 * t4484 / F::new(48.0) + t8634 * t833 / F::new(48.0) + F::new(35.0) / F::new(216.0) * t4425 - F::new(35.0) / F::new(216.0) * t4430 - F::new(35.0) / F::new(108.0) * t4443 + t8641 + t8643 - F::new(7.0) / F::new(288.0) * t4454 + t8646 + t3066 * t8649 / F::new(24.0) - t8654 * t2373 / F::new(24.0) + F::new(7.0) / F::new(144.0) * t4467 + F::new(7.0) / F::new(72.0) * t4469 + t8659 * t6164 / F::new(48.0) + t8664 - t8666;
    t8667
}
