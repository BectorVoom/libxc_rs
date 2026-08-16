//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1266/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1266<F: Float>(t10627: F, t1880: F, t23335: F, t6066: F, t1710: F) -> (F, F, F) {
    let t32889 = t10627 * t1880;
    let t32892 = F::cast_from(0.14300195980740170668e1_f64) * t23335 * t6066 * t32889;
    let t32893 = t10627 * t1710;
    (t32889, t32892, t32893)
}
