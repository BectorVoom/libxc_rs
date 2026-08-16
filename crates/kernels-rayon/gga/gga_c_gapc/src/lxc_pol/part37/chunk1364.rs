//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 1364/1445 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk1364(t12288: f64, t23723: f64, t3622: f64, t9375: f64, t10088: f64, t11046: f64, t33179: f64, t33182: f64, t33185: f64, t33187: f64, t33190: f64, t33193: f64, t33195: f64, t33197: f64, t33200: f64, t33203: f64, t33205: f64) -> (f64, f64, f64, f64) {
    let t36472 = 12.0_f64 * t23723 * t12288;
    let t36474 = 2.0_f64 * t9375 * t3622;
    let t36483 = 2.0_f64 * t11046 * t10088;
    let t36506 = 0.10298285674687440379e-4_f64 * t33179 + 0.1374296967252737644e-5_f64 * t33182 - 0.22509399720615334744e-7_f64 * t33185 - 0.33147827249531850013e-7_f64 * t33187 - 0.45018799441230669488e-7_f64 * t33190 + 0.33816362383187442026e-5_f64 * t33193 + 0.9275345110817126956e-4_f64 * t33195 - 0.9275345110817126956e-4_f64 * t33197 - 0.49163213094075520836e-8_f64 * t33200 + 0.64085799349094910026e-6_f64 * t33203 + 0.67528199161846004232e-6_f64 * t33205;
    (t36472, t36474, t36483, t36506)
}
