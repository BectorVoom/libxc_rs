//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 948/1426 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk948<F: Float>(t9898: F, t9901: F, t9904: F, t9908: F, t9910: F, t9914: F, t9917: F, t9924: F, t9930: F, t9935: F, t9937: F, t9939: F, t9941: F) -> F {
    let t10991 = F::new(0.25781643416302550011e-8) * t9898 + F::new(0.42270452978984302532e-6) * t9901 + F::new(0.12380169846338434109e-5) * t9904 - F::new(0.84410248952307505288e-7) * t9908 - F::new(0.16882049790461501058e-6) * t9910 - F::new(0.84410248952307505288e-7) * t9914 - F::new(0.10005428175813516294e-7) * t9917 + F::new(0.20010856351627032588e-7) * t9924 - F::new(0.14591249423061377928e-8) * t9930 + F::new(0.49239311888846044752e-7) * t9935 + F::new(0.21642471925239962898e-3) * t9937 + F::new(0.2318836277704281739e-4) * t9939 + F::new(0.80043425406508130349e-7) * t9941;
    t10991
}
