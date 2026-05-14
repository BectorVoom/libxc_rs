//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 790/1135 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk790<F: Float>(t3478: F, t575: F, t1104: F, t1615: F, t1112: F, t1617: F, t3537: F, t687: F, t2011: F, t8622: F, t8626: F, t8629: F, t8632: F, t8634: F, t8637: F, t8641: F, t8645: F, t8647: F, t8650: F, t8657: F, t8660: F, t8663: F) -> (F, F, F, F, F, F) {
    let t10526 = t3478 * t575;
    let t10529 = t1104 * t1615;
    let t10538 = t1112 * t1617;
    let t10541 = t3537 * t687;
    let t10544 = t1112 * t2011;
    let t10560 = 0.32827263770475230566e-7 * t8622 - 0.11594181388521408694e-4 * t8626 - 0.55603792169291016668e-2 * t8629 - 0.55603792169291016668e-2 * t8632 - 0.13913017666225690434e-3 * t8634 - 0.22510123728325872388e-6 * t8637 - 0.46497498276882732785e-5 * t8641 - 0.34752370105806885418e-3 * t8645 - 0.24326659074064819793e-2 * t8647 + 0.42270452978984302532e-6 * t8650 + 0.12328882118870421572e-6 * t8657 - 0.55603792169291016668e-2 * t8660 + 0.24326659074064819792e-2 * t8663;
    (t10526, t10529, t10538, t10541, t10544, t10560)
}
