//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1076/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1076<F: Float>(t5192: F, t6552: F, t1188: F, t24375: F, t3520: F, t1196: F, t1765: F, t20400: F, t5197: F, t6535: F, t6556: F, t12485: F) -> (F, F, F, F, F, F) {
    let t24478 = F::cast_from(0.17544670867903938621e1_f64) * t5192 * t6552;
    let t24480 = t3520 * t24375 * t1188;
    let t24482 = F::cast_from(0.35089341735807877242e1_f64) * t1196 * t24480;
    let t24484 = F::cast_from(0.17544670867903938621e1_f64) * t20400 * t1765;
    let t24488 = t5197 * t6535;
    let t24490 = F::cast_from(0.35089341735807877242e1_f64) * t1196 * t24488;
    let t24492 = F::cast_from(0.51947577317044391276e2_f64) * t5192 * t6556;
    let t24493 = t12485 * t24375;
    (t24478, t24482, t24484, t24490, t24492, t24493)
}
