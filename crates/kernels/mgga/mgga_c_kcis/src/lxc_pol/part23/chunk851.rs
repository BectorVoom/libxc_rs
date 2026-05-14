//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 851/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk851<F: Float>(t17063: F, t17252: F, t509: F, t552: F, t557: F, t303: F, t1014: F, t5872: F, t1928: F, t4161: F, t4165: F, t4160: F, t2820: F, t5659: F, t86: F, t5664: F) -> (F, F, F, F, F, F, F) {
    let t17253 = t17063 + t17252;
    let t17254 = t509 * t17253;
    let t17255 = t17254 * t552;
    let t17256 = t17255 * t557;
    let t17257 = t303 * t17256;
    let t17259 = t1014 * t5872;
    let t17260 = 0.33163888888888888888e-2 * t17259;
    let t17261 = t4161 * t1928;
    let t17262 = t17261 * t4165;
    let t17263 = t4160 * t17262;
    let t17266 = t86 * t2820 * t5659;
    let t17267 = t17266 * t5664;
    (t17253, t17254, t17257, t17259, t17260, t17263, t17267)
}
