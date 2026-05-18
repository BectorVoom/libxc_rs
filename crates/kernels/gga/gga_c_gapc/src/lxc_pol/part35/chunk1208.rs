//! GGA_C_GAPC lxc pol — lxc_pol part 35 (v4rho2sigma2_14) CSE chunk 1208/1307 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part35_v4rho2sigma2_14_chunk1208<F: Float>(t436: F, t8775: F, t8776: F, t34940: F, t34942: F, t34946: F, t34949: F, t34951: F, t34954: F, t34956: F, t34958: F, t34960: F, t34962: F) -> F {
    let t34965 = t8775 * t436 * t8776;
    let t34967 = F::new(0.80045999977926802213e-7) * t34940 - F::new(0.20259111355493285149e-5) * t34942 + F::new(0.88397049170382309323e-8) * t34946 - F::new(0.90579542097823505428e-7) * t34949 - F::new(0.25301920572916666668e-5) * t34951 - F::new(0.49190053374354708085e-8) * t34954 - F::new(0.13259557375557346398e-6) * t34956 - F::new(0.13259557375557346398e-6) * t34958 - F::new(0.6629778687778673199e-7) * t34960 - F::new(0.90579542097823505428e-7) * t34962 - F::new(0.22510123728325872388e-7) * t34965;
    t34967
}
