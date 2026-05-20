//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 588/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk588<F: Float>(t1179: F, t1749: F, t1187: F, t1757: F, t3358: F, t3415: F, t3503: F, t3510: F, t5044: F, t5049: F, t5054: F, t5058: F, t5072: F, t5080: F, t5088: F, t5090: F, t5093: F, t5096: F, t5099: F, t5102: F) -> (F, F, F) {
    let t5158 = t1749 * t1179;
    let t5163 = t1757 * t1187;
    let t5180 = -F::new(0.1294625e1) * t5072 + F::new(0.258925e1) * t5080 + t3503 - F::cast_from(0.10064166666666666667e0_f64) * t3358 - F::cast_from(0.10064166666666666667e0_f64) * t5044 - F::cast_from(0.20128333333333333333e0_f64) * t5049 + F::new(0.60385e0) * t5054 + F::new(0.301925e0) * t5058 + F::new(0.82524375e-1) * t5088 + F::new(0.16504875e0) * t5090 + t3510 - F::new(0.5519e-1) * t3415 - F::new(0.5519e-1) * t5093 - F::new(0.27595e-1) * t5096 + F::new(0.16557e0) * t5099 + F::new(0.82785e-1) * t5102;
    (t5158, t5163, t5180)
}
