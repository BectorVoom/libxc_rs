//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 927/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk927<F: Float>(t1359: F, t614: F, t1969: F, t379: F, t1361: F, t1637: F, t1349: F, t5766: F, t5769: F, t5843: F, t28: F, t23884: F, t525: F, t165: F, t1360: F, t2228: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t24102 = t1359 * t614;
    let t24104 = t1969 * t24102 * t379;
    let t24116 = t1637 * t1361;
    let t24118 = 2.0 / 27.0 * t1349 * t24116;
    let t24119 = t5766 * t5769;
    let t24121 = t5843 * t614;
    let t24122 = t28 * t24121;
    let t24125 = t525 * t23884;
    let t24126 = t24125 * t165;
    let t24127 = t28 * t24126;
    let t24130 = t1360 * t2228;
    (t24102, t24104, t24116, t24118, t24119, t24121, t24122, t24125, t24126, t24127, t24130)
}
