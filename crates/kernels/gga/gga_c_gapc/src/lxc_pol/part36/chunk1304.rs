//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 1304/1328 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk1304<F: Float>(t33956: F, t33962: F, t33967: F, t33969: F, t33972: F, t33975: F, t33978: F, t33980: F, t33983: F, t33988: F, t33991: F, t34036: F, t34038: F, t34043: F, t34046: F, t34048: F, t34050: F, t34052: F, t34054: F, t34056: F, t34060: F, t34062: F) -> (F, F) {
    let t37950 = -F::new(0.1348042442506961251e-6) * t33956 - F::new(0.40083661544871514617e-6) * t33962 + F::new(0.1672914890006736473e-7) * t33967 - F::new(0.5060221354166666667e-5) * t33969 + F::new(0.4637672555408563478e-4) * t33972 - F::new(0.14339270485772026911e-8) * t33975 + F::new(0.95595136571813512741e-9) * t33978 + F::new(0.2318836277704281739e-4) * t33980 - F::new(0.5462579232675057871e-9) * t33983 - F::new(0.99511007074824895497e-6) * t33988 - F::new(0.36620703859188537988e-5) * t33991;
    let t37976 = -F::new(0.23333242910879629631e-3) * t34036 + F::new(0.2318836277704281739e-4) * t34038 - F::new(0.12309827972211511188e-7) * t34043 - F::new(0.6154913986105755594e-8) * t34046 + F::new(0.39777392699438220015e-6) * t34048 - F::new(0.41267232821128113697e-4) * t34050 + F::new(0.6403474032520650428e-6) * t34052 + F::new(0.57211390956563975807e-5) * t34054 + F::new(0.2813674965076916843e-7) * t34056 + F::new(0.93146396372185817726e-9) * t34060 + F::new(0.99041358770707472872e-5) * t34062;
    (t37950, t37976)
}
