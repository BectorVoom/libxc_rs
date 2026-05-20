//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2054/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2054<F: Float>(t98206: F, t2689: F, t27936: F, t13857: F, t94564: F, t25978: F, t5629: F, t1885: F, t94459: F, t26024: F, t5661: F, t14054: F, t25986: F, t2661: F) -> (F, F, F, F, F, F, F) {
    let t98207 = F::cast_from(0.10164000561857065645e-2_f64) * t98206;
    let t98218 = t2689 * t27936;
    let t98220 = t94564 * t13857;
    let t98222 = t25978 * t5629;
    let t98224 = t94459 * t1885;
    let t98226 = t26024 * t5661;
    let t98227 = F::cast_from(0.40015750243531754508e-2_f64) * t98226;
    let t98229 = t2661 * t25986 * t14054;
    (t98207, t98218, t98220, t98222, t98224, t98227, t98229)
}
