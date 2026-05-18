//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 1293/1327 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk1293<F: Float>(t36009: F, t36011: F, t36013: F, t36017: F, t36020: F, t36022: F, t36025: F, t36028: F, t36030: F, t36034: F, t36037: F, t36040: F, t36042: F, t36044: F) -> F {
    let t37614 = -F::new(0.65659062294300875668e-4) * t36009 + F::new(0.28452260327530379456e-3) * t36011 + F::new(0.28452260327530379456e-3) * t36013 + F::new(0.58714905980103539484e-5) * t36017 - F::new(0.32829531147150437834e-4) * t36020 + F::new(0.32829531147150437834e-4) * t36022 + F::new(0.93943849568165663176e-5) * t36025 + F::new(0.65659062294300875668e-4) * t36028 - F::new(0.43840463131810642815e-4) * t36030 + F::new(0.65659062294300875668e-4) * t36034 - F::new(0.32829531147150437834e-4) * t36037 - F::new(0.45596571037708941436e-6) * t36040 + F::new(0.75872694206747678549e-3) * t36042 - F::new(0.43316742485823494364e-5) * t36044;
    t37614
}
