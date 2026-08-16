//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1284/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1284<F: Float>(t3358: F, t3415: F, t3459: F, t3466: F, t5044: F, t5049: F, t5054: F, t5058: F, t5072: F, t5080: F, t5088: F, t5090: F, t5093: F, t5096: F, t5099: F, t5102: F) -> F {
    let t5142 = -F::cast_from(0.17648625e1_f64) * t5072 + F::cast_from(0.3529725e1_f64) * t5080 + t3459 - F::cast_from(0.17215833333333333333e0_f64) * t3358 - F::cast_from(0.17215833333333333333e0_f64) * t5044 - F::cast_from(0.34431666666666666667e0_f64) * t5049 + F::cast_from(0.103295e1_f64) * t5054 + F::cast_from(0.516475e0_f64) * t5058 + F::cast_from(0.31558125e0_f64) * t5088 + F::cast_from(0.6311625e0_f64) * t5090 + t3466 - F::cast_from(0.69463333333333333333e-1_f64) * t3415 - F::cast_from(0.69463333333333333333e-1_f64) * t5093 - F::cast_from(0.34731666666666666667e-1_f64) * t5096 + F::cast_from(0.20839e0_f64) * t5099 + F::cast_from(0.104195e0_f64) * t5102;
    t5142
}
