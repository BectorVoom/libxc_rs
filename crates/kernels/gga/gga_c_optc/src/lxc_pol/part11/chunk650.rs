//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 650/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk650<F: Float>(t450: F, t5313: F, t1111: F, t1121: F, t1133: F, t1503: F, t1520: F, t431: F, t4310: F, t4334: F, t4369: F, t4381: F, t451: F, t5276: F, t5280: F, t5286: F, t5290: F, t5298: F, t5302: F) -> (F, F) {
    let t5314 = t450 * t5313;
    let t5317 = F::new(11.0) / F::new(108.0) * t5276 * t431 + F::new(0.9176114905888133291e-1) * t5280 * t451 - t4310 * t1503 / F::new(54.0) + t1111 * t5286 / F::new(288.0) + t1111 * t5290 / F::new(216.0) + F::new(0.24147670804968771818e-2) * t4381 + F::new(0.47333755318775392234e-1) * t4334 - F::new(0.19318136643975017455e-1) * t4369 * t1520 + F::new(0.18110753103726578864e-2) * t1133 * t5298 + F::new(0.30184588506210964773e-2) * t1133 * t5302 + F::new(0.35500316489081544176e-1) * t1121 * t5314;
    (t5314, t5317)
}
