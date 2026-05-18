//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 1364/1445 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk1364<F: Float>(t12288: F, t23723: F, t3622: F, t9375: F, t10088: F, t11046: F, t33179: F, t33182: F, t33185: F, t33187: F, t33190: F, t33193: F, t33195: F, t33197: F, t33200: F, t33203: F, t33205: F) -> (F, F, F, F) {
    let t36472 = F::new(12.0) * t23723 * t12288;
    let t36474 = F::new(2.0) * t9375 * t3622;
    let t36483 = F::new(2.0) * t11046 * t10088;
    let t36506 = F::new(0.10298285674687440379e-4) * t33179 + F::new(0.1374296967252737644e-5) * t33182 - F::new(0.22509399720615334744e-7) * t33185 - F::new(0.33147827249531850013e-7) * t33187 - F::new(0.45018799441230669488e-7) * t33190 + F::new(0.33816362383187442026e-5) * t33193 + F::new(0.9275345110817126956e-4) * t33195 - F::new(0.9275345110817126956e-4) * t33197 - F::new(0.49163213094075520836e-8) * t33200 + F::new(0.64085799349094910026e-6) * t33203 + F::new(0.67528199161846004232e-6) * t33205;
    (t36472, t36474, t36483, t36506)
}
