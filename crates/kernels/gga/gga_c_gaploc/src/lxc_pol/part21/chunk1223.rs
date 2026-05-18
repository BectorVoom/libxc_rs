//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1223/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1223<F: Float>(t32697: F, t11135: F, t5552: F, t2728: F, t8440: F, t16705: F, t3459: F, t24303: F, t977: F, t10805: F, t5559: F, t841: F) -> (F, F, F, F, F, F) {
    let t32698 = F::new(0.96131577876777803547e-3) * t32697;
    let t32708 = F::new(4.0) * t5552 * t11135;
    let t32713 = F::new(2.0) * t8440 * t2728;
    let t32715 = F::new(2.0) * t16705 * t3459;
    let t32716 = t24303 * t977;
    let t32719 = F::new(12.0) * t5559 * t10805 * t841;
    (t32698, t32708, t32713, t32715, t32716, t32719)
}
