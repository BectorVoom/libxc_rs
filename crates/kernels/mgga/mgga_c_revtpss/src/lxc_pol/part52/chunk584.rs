//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 584/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk584<F: Float>(t141: F, t5098: F, t1145: F, t5056: F, t3358: F, t3402: F, t3414: F, t3415: F, t5044: F, t5049: F, t5054: F, t5058: F, t5072: F, t5080: F, t5088: F, t5090: F, t5093: F, t5096: F) -> (F, F, F) {
    let t5099 = t141 * t5098;
    let t5101 = t1145 * t5056;
    let t5102 = t141 * t5101;
    let t5104 = -F::new(0.9494625e0) * t5072 + F::new(0.1898925e1) * t5080 + t3402 - F::cast_from(0.99655555555555555557e-1_f64) * t3358 - F::cast_from(0.99655555555555555557e-1_f64) * t5044 - F::cast_from(0.19931111111111111111e0_f64) * t5049 + F::cast_from(0.59793333333333333334e0_f64) * t5054 + F::cast_from(0.29896666666666666667e0_f64) * t5058 + F::new(0.15358125e0) * t5088 + F::new(0.3071625e0) * t5090 + t3414 - F::cast_from(0.54771111111111111111e-1_f64) * t3415 - F::cast_from(0.54771111111111111111e-1_f64) * t5093 - F::cast_from(0.27385555555555555556e-1_f64) * t5096 + F::cast_from(0.16431333333333333333e0_f64) * t5099 + F::cast_from(0.82156666666666666667e-1_f64) * t5102;
    (t5099, t5102, t5104)
}
