//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 598/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk598<F: Float>(t1160: F, t1737: F, t1168: F, t1745: F, t3358: F, t3415: F, t3459: F, t3466: F, t5044: F, t5049: F, t5054: F, t5058: F, t5072: F, t5080: F, t5088: F, t5090: F, t5093: F, t5096: F, t5099: F, t5102: F) -> (F, F, F) {
    let t5120 = t1737 * t1160;
    let t5125 = t1745 * t1168;
    let t5142 = -F::new(0.17648625e1) * t5072 + F::new(0.3529725e1) * t5080 + t3459 - F::new(0.17215833333333333333e0) * t3358 - F::new(0.17215833333333333333e0) * t5044 - F::new(0.34431666666666666667e0) * t5049 + F::new(0.103295e1) * t5054 + F::new(0.516475e0) * t5058 + F::new(0.31558125e0) * t5088 + F::new(0.6311625e0) * t5090 + t3466 - F::new(0.69463333333333333333e-1) * t3415 - F::new(0.69463333333333333333e-1) * t5093 - F::new(0.34731666666666666667e-1) * t5096 + F::new(0.20839e0) * t5099 + F::new(0.104195e0) * t5102;
    (t5120, t5125, t5142)
}
