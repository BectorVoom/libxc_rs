//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1450/1464 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1450<F: Float>(t12255: F, t12318: F, t1716: F, t1897: F, t1901: F, t2042: F, t2508: F, t29354: F, t32355: F, t32360: F, t32363: F, t32370: F, t32394: F, t32398: F, t32400: F, t32408: F, t32411: F, t3722: F, t39107: F, t5227: F) -> F {
    let t39420 = F::cast_from(0.15381052460284448567e-1_f64) * t1897 * t1901 * t39107 - t32355 - t32360 + t29354 + F::cast_from(0.17090058289204942853e-2_f64) * t5227 * t12318 - t32363 + F::cast_from(0.76905262301422242837e-2_f64) * t2508 * t2042 * t3722 - F::cast_from(0.23071578690426672851e-1_f64) * t2508 * t12255 * t1716 + t32370 - t32394 + t32398 - t32400 - t32408 + t32411;
    t39420
}
