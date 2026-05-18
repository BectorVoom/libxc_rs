//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 683/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk683<F: Float>(t24974: F, t24987: F, t28722: F, t28727: F, t28732: F, t28739: F, t28744: F, t28749: F, t28753: F, t28758: F, t28762: F, t28765: F) -> F {
    let t28897 = -t28722 - t24974 / F::new(12.0) - t28727 / F::new(12.0) - t28732 / F::new(12.0) - F::new(2.0) / F::new(3.0) * t24987 - F::new(3.0) / F::new(8.0) * t28739 - t28744 / F::new(2.0) + t28749 / F::new(6.0) + t28753 / F::new(6.0) - t28758 / F::new(3.0) - t28762 / F::new(3.0) - t28765 / F::new(3.0);
    t28897
}
