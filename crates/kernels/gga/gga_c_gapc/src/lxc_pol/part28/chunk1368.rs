//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 1368/1429 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk1368<F: Float>(t12288: F, t23723: F, t3622: F, t9375: F, t12149: F, t1616: F, t687: F, t1611: F, t10088: F, t11046: F, t33179: F, t33182: F, t33185: F, t33187: F, t33190: F, t33193: F, t33195: F, t33197: F, t33200: F, t33203: F, t33205: F) -> (F, F, F, F, F, F) {
    let t36472 = F::new(12.0) * t23723 * t12288;
    let t36474 = F::new(2.0) * t9375 * t3622;
    let t36479 = F::new(4.0) * t1616 * t12149 * t687;
    let t36481 = F::new(2.0) * t1611 * t12149;
    let t36483 = F::new(2.0) * t11046 * t10088;
    let t36506 = F::cast_from(0.10298285674687440379e-4_f64) * t33179 + F::cast_from(0.1374296967252737644e-5_f64) * t33182 - F::cast_from(0.22509399720615334744e-7_f64) * t33185 - F::cast_from(0.33147827249531850013e-7_f64) * t33187 - F::cast_from(0.45018799441230669488e-7_f64) * t33190 + F::cast_from(0.33816362383187442026e-5_f64) * t33193 + F::cast_from(0.9275345110817126956e-4_f64) * t33195 - F::cast_from(0.9275345110817126956e-4_f64) * t33197 - F::cast_from(0.49163213094075520836e-8_f64) * t33200 + F::cast_from(0.64085799349094910026e-6_f64) * t33203 + F::cast_from(0.67528199161846004232e-6_f64) * t33205;
    (t36472, t36474, t36479, t36481, t36483, t36506)
}
