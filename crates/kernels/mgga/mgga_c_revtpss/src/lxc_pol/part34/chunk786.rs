//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 786/1196 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk786<F: Float>(t406: F, t12295: F, t11335: F, t281: F, t414: F, t3475: F, t431: F, t426: F, t1159: F, t3478: F, t434: F, t3519: F, t444: F, t439: F, t1178: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t12331 = 1.0/pow_3_2(t406);
    let t12349 = 0.93011851851851851854e0 * t12295;
    let t12351 = t281 * t11335 * t414;
    let t12352 = 0.36514074074074074075e0 * t12351;
    let t12367 = 0.28842592592592592592e-1 * t12295;
    let t12382 = 0.55403703703703703703e-1 * t12295;
    let t12397 = 0.53272592592592592592e-1 * t12295;
    let t12428 = 1.0 / t3475 / t431;
    let t12429 = t426 * t12428;
    let t12459 = 0.16068111111111111111e1 * t12295;
    let t12460 = 0.46308888888888888888e0 * t12351;
    let t12469 = 1.0 / t3475 / t1159;
    let t12470 = t426 * t12469;
    let t12472 = 1.0 / t3478 / t434;
    let t12485 = 1.0 / t3519 / t444;
    let t12486 = t439 * t12485;
    let t12542 = 0.93932222222222222223e0 * t12295;
    let t12543 = 0.36793333333333333333e0 * t12351;
    let t12552 = 1.0 / t3519 / t1178;
    (t12331, t12349, t12352, t12367, t12382, t12397, t12429, t12459, t12460, t12470, t12472, t12485, t12486, t12542, t12543, t12552)
}
