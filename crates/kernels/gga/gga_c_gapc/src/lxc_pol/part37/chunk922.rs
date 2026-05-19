//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 922/1445 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk922<F: Float>(t3478: F, t575: F, t1104: F, t1615: F, t1112: F, t1617: F, t3537: F, t687: F, t2011: F, t8622: F, t8626: F, t8629: F, t8632: F, t8634: F, t8637: F, t8641: F, t8645: F, t8647: F, t8650: F, t8657: F, t8660: F, t8663: F) -> (F, F, F, F, F, F) {
    let t10526 = t3478 * t575;
    let t10529 = t1104 * t1615;
    let t10538 = t1112 * t1617;
    let t10541 = t3537 * t687;
    let t10544 = t1112 * t2011;
    let t10560 = F::cast_from(0.32827263770475230566e-7_f64) * t8622 - F::cast_from(0.11594181388521408694e-4_f64) * t8626 - F::cast_from(0.55603792169291016668e-2_f64) * t8629 - F::cast_from(0.55603792169291016668e-2_f64) * t8632 - F::cast_from(0.13913017666225690434e-3_f64) * t8634 - F::cast_from(0.22510123728325872388e-6_f64) * t8637 - F::cast_from(0.46497498276882732785e-5_f64) * t8641 - F::cast_from(0.34752370105806885418e-3_f64) * t8645 - F::cast_from(0.24326659074064819793e-2_f64) * t8647 + F::cast_from(0.42270452978984302532e-6_f64) * t8650 + F::cast_from(0.12328882118870421572e-6_f64) * t8657 - F::cast_from(0.55603792169291016668e-2_f64) * t8660 + F::cast_from(0.24326659074064819792e-2_f64) * t8663;
    (t10526, t10529, t10538, t10541, t10544, t10560)
}
