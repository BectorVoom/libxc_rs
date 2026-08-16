//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 975/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk975<F: Float>(t1114: F, t6159: F, t6154: F, t2362: F, t2397: F, t3083: F, t2373: F, t3066: F, t4390: F, t4425: F, t4430: F, t4443: F, t4454: F, t4467: F, t4469: F, t4484: F, t6164: F, t833: F, t8629: F, t8634: F, t8641: F, t8643: F, t8646: F, t8649: F, t8654: F) -> F {
    let t8659 = t1114 * t6159;
    let t8662 = t1114 * t6154;
    let t8664 = F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t8662 * t2362;
    let t8666 = F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t3083 * t2397;
    let t8667 = t8629 * t4390 / F::cast_from(24.0_f64) + t8629 * t4484 / F::cast_from(48.0_f64) + t8634 * t833 / F::cast_from(48.0_f64) + F::cast_from(35.0_f64) / F::cast_from(216.0_f64) * t4425 - F::cast_from(35.0_f64) / F::cast_from(216.0_f64) * t4430 - F::cast_from(35.0_f64) / F::cast_from(108.0_f64) * t4443 + t8641 + t8643 - F::cast_from(7.0_f64) / F::cast_from(288.0_f64) * t4454 + t8646 + t3066 * t8649 / F::cast_from(24.0_f64) - t8654 * t2373 / F::cast_from(24.0_f64) + F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t4467 + F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t4469 + t8659 * t6164 / F::cast_from(48.0_f64) + t8664 - t8666;
    t8667
}
