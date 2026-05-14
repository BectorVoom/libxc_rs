//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1383/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1383<F: Float>(t193: F, t5225: F, t89: F, t98359: F, t113581: F, t1212: F, t126389: F, t24976: F, t6317: F, t126397: F, t24981: F, t126401: F, t28755: F, t1091: F, t24980: F, t28816: F) -> (F, F, F, F, F, F) {
    let t127796 = t89 * t193 * t98359 * t5225;
    let t127800 = t89 * t193 * t113581 * t1212;
    let t127803 = t6317 * t24976 * t126389;
    let t127806 = t6317 * t24981 * t126397;
    let t127808 = t28755 * t24981 * t126401;
    let t127812 = t24980 * t24981 * t28816 * t1091;
    (t127796, t127800, t127803, t127806, t127808, t127812)
}
