//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 827/1274 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk827<F: Float>(t141: F, t5095: F, t1145: F, t5052: F, t5056: F, t3358: F, t3402: F, t3414: F, t3415: F, t5044: F, t5049: F, t5054: F, t5058: F, t5072: F, t5080: F, t5088: F, t5090: F, t5093: F) -> (F, F, F, F, F, F) {
    let t5096 = t141 * t5095;
    let t5098 = t1145 * t5052;
    let t5099 = t141 * t5098;
    let t5101 = t1145 * t5056;
    let t5102 = t141 * t5101;
    let t5104 = -0.9494625e0 * t5072 + 0.1898925e1 * t5080 + t3402 - 0.99655555555555555557e-1 * t3358 - 0.99655555555555555557e-1 * t5044 - 0.19931111111111111111e0 * t5049 + 0.59793333333333333334e0 * t5054 + 0.29896666666666666667e0 * t5058 + 0.15358125e0 * t5088 + 0.3071625e0 * t5090 + t3414 - 0.54771111111111111111e-1 * t3415 - 0.54771111111111111111e-1 * t5093 - 0.27385555555555555556e-1 * t5096 + 0.16431333333333333333e0 * t5099 + 0.82156666666666666667e-1 * t5102;
    (t5096, t5098, t5099, t5101, t5102, t5104)
}
