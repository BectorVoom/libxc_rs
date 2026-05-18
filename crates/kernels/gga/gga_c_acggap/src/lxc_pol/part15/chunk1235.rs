//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1235/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1235<F: Float>(t32635: F, t35055: F, t35076: F, t35100: F, t37372: F, t37373: F, t37374: F, t37379: F, t37380: F, t37381: F, t37382: F, t39686: F, t39690: F, t39693: F, t39696: F, t39700: F, t39705: F, t39709: F) -> F {
    let t41759 = -F::new(0.31448092289604152068e-2) * t35055 + F::new(0.85748036236139473944e-3) * t39686 + F::new(0.66040993808168719343e-1) * t39690 + F::new(0.36675e0) * t39693 + F::new(0.2750625e0) * t39696 - t37372 - t37373 - t37374 - t32635 - F::new(77.0) / F::new(144.0) * t35076 - F::new(0.7640625e-2) * t39700 + t37379 + t37380 - t37381 - t37382 - F::new(0.25724410870841842183e-2) * t35100 - F::new(0.42874018118069736972e-2) * t39705 + F::new(0.56606566121287473724e-1) * t39709;
    t41759
}
