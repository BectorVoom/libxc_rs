//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 1159/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk1159<F: Float>(t10088: F, t11046: F, t33179: F, t33182: F, t33185: F, t33187: F, t33190: F, t33193: F, t33195: F, t33197: F, t33200: F, t33203: F, t33205: F, t33150: F, t33154: F, t33156: F, t33160: F, t33162: F, t33165: F, t33167: F, t33170: F, t33173: F, t33175: F) -> (F, F) {
    let t36483 = 2.0 * t11046 * t10088;
    let t36506 = 0.10298285674687440379e-4 * t33179 + 0.1374296967252737644e-5 * t33182 - 0.22509399720615334744e-7 * t33185 - 0.33147827249531850013e-7 * t33187 - 0.45018799441230669488e-7 * t33190 + 0.33816362383187442026e-5 * t33193 + 0.9275345110817126956e-4 * t33195 - 0.9275345110817126956e-4 * t33197 - 0.49163213094075520836e-8 * t33200 + 0.64085799349094910026e-6 * t33203 + 0.67528199161846004232e-6 * t33205;
    let t36507 = -0.21135226489492151266e-6 * t33150 + 0.80189736504692130024e-6 * t33154 + 0.63307686714230628966e-7 * t33156 - 0.99041358770707472873e-5 * t33160 - 0.13259130899812740005e-6 * t33162 - 0.44197102999375800018e-8 * t33165 - 0.66295654499063700026e-7 * t33167 + 0.43440462632258606772e-4 * t33170 + 0.11372686522837130914e-5 * t33173 + 0.10298285674687440379e-4 * t33175 + t36506;
    (t36483, t36507)
}
