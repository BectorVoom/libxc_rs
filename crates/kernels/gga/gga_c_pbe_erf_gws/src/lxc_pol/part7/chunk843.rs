//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 843/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk843<F: Float>(t17260: F, t4952: F, t587: F, t610: F, t16718: F, t7435: F, t5283: F, t597: F, t5544: F, t1820: F, t1821: F, t4976: F, t562: F, t1827: F, t16699: F, t17246: F, t17251: F, t17255: F, t17257: F, t17259: F) -> (F, F, F, F, F, F, F) {
    let t17264 = 128.0 / 81.0 * t587 * t17260 * t4952 * t610;
    let t17267 = 64.0 / 27.0 * t587 * t7435 * t16718;
    let t17268 = t5283 * t597;
    let t17270 = t587 * t17268 * t5544;
    let t17271 = 32.0 / 27.0 * t17270;
    let t17275 = 32.0 / 45.0 * t1820 * t1821 * t4976 * t562;
    let t17279 = 16.0 / 45.0 * t587 * t1827 * t4976 * t610;
    let t17282 = 32.0 / 45.0 * t587 * t1821 * t16699;
    let t17283 = t17246 + t17251 + t17255 - t17257 + t17259 - t17264 + t17267 - t17271 + t17275 - t17279 - t17282;
    (t17264, t17267, t17271, t17275, t17279, t17282, t17283)
}
