//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 1048/1427 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk1048<F: Float>(t11459: F, t11373: F, t11382: F, t11385: F, t11392: F, t11403: F, t11445: F, t11453: F, t12086: F, t12087: F, t12090: F, t12093: F, t12094: F, t12095: F, t12096: F, t12097: F, t12098: F, t12099: F, t12100: F, t12101: F) -> F {
    let t12104 = F::new(0.10110318318802209383e-5) * t11459;
    let t12105 = -F::new(0.90579542097823505425e-7) * t11373 + t12086 + t12087 - F::new(0.4419852458519115466e-8) * t11382 - F::new(0.66297786877786731988e-7) * t11385 + t12090 + F::new(0.57970906942607043474e-5) * t11392 - F::new(0.14340192936791314022e-8) * t11403 + t12093 + t12094 - t12095 - t12096 - t12097 + t12098 - t12099 - t12100 - t12101 - F::new(0.64087860648527174255e-6) * t11445 + F::new(0.98332751566569010434e-8) * t11453 + t12104;
    t12105
}
