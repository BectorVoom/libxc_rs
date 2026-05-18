//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1182/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1182<F: Float>(t1734: F, t7380: F, t7381: F, t1886: F, t7605: F, t2041: F, t5598: F, t6167: F, t31612: F, t31619: F, t31625: F, t31627: F, t31629: F, t31632: F, t31644: F, t31646: F, t35910: F, t35912: F, t35914: F, t37757: F, t37758: F) -> F {
    let t40295 = t7380 * t7381 * t1734;
    let t40297 = t7605 * t1886;
    let t40299 = t2041 * t5598;
    let t40301 = t2041 * t6167;
    let t40305 = -t37757 - t37758 + F::new(0.85748036236139473944e-3) * t31612 + F::new(0.94344276868812456205e-2) * t31619 + F::new(0.12862205435420921092e-2) * t31625 + F::new(0.25724410870841842184e-2) * t31627 + F::new(0.6431102717710460546e-2) * t31629 - F::new(0.40015750243531754508e-2) * t31632 - t40295 / F::new(64.0) + F::new(0.85748036236139473945e-2) * t40297 - t40299 / F::new(48.0) - t40301 / F::new(48.0) - F::new(0.11337795902333997111e-1) * t31644 - F::new(0.16006300097412701803e-1) * t31646 + t35910 + t35912 + t35914;
    t40305
}
