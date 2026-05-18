//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 1063/1445 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk1063<F: Float>(t11373: F, t11382: F, t11385: F, t11392: F, t11403: F, t11445: F, t11453: F, t12086: F, t12087: F, t12090: F, t12093: F, t12094: F, t12095: F, t12096: F, t12097: F, t12098: F, t12099: F, t12100: F, t12101: F, t12104: F) -> F {
    let t12607 = -F::new(0.90579542097823505428e-7) * t11373 + t12086 + t12087 - F::new(0.44198524585191154661e-8) * t11382 - F::new(0.6629778687778673199e-7) * t11385 + t12090 + F::new(0.57970906942607043475e-5) * t11392 - F::new(0.14340192936791314021e-8) * t11403 + t12093 + t12094 - t12095 - t12096 - t12097 + t12098 - t12099 - t12100 - t12101 - F::new(0.64087860648527174257e-6) * t11445 + F::new(0.98332751566569010432e-8) * t11453 + t12104;
    t12607
}
