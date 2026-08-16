//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 219/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk219<F: Float>(t281: F, t283: F, t926: F, t240: F, t346: F, t906: F, t141: F, t908: F, t919: F, t921: F, t924: F, t290: F) -> (F, F, F, F, F, F, F) {
    let t928 = t281 * t926 * t283;
    let t929 = F::cast_from(0.82156666666666666667e-1_f64) * t928;
    let t930 = t240 * t346;
    let t931 = t930 * t906;
    let t932 = t141 * t931;
    let t934 = F::cast_from(0.1898925e1_f64) * t919 - t921 - F::cast_from(0.29896666666666666667e0_f64) * t908 + F::cast_from(0.3071625e0_f64) * t924 - t929 - F::cast_from(0.82156666666666666667e-1_f64) * t932;
    let t935 = F::cast_from(1.0_f64) / t290;
    (t928, t929, t930, t931, t932, t934, t935)
}
