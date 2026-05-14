//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 732/1286 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk732<F: Float>(t3358: F, t3415: F, t3503: F, t3510: F, t5044: F, t5049: F, t5054: F, t5058: F, t5072: F, t5080: F, t5088: F, t5090: F, t5093: F, t5096: F, t5099: F, t5102: F) -> (F,) {
    let t5180 = -0.1294625e1 * t5072 + 0.258925e1 * t5080 + t3503 - 0.10064166666666666667e0 * t3358 - 0.10064166666666666667e0 * t5044 - 0.20128333333333333333e0 * t5049 + 0.60385e0 * t5054 + 0.301925e0 * t5058 + 0.82524375e-1 * t5088 + 0.16504875e0 * t5090 + t3510 - 0.5519e-1 * t3415 - 0.5519e-1 * t5093 - 0.27595e-1 * t5096 + 0.16557e0 * t5099 + 0.82785e-1 * t5102;
    (t5180,)
}
