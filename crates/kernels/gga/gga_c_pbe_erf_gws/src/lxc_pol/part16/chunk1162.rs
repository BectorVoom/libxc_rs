//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1162/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1162<F: Float>(t54113: F, t54117: F, t54126: F, t54128: F, t54107: F, t54109: F, t54111: F, t54115: F, t54120: F, t54122: F, t54124: F, t54130: F, t54135: F, t54152: F, t51252: F, t54133: F, t54137: F, t54139: F, t54142: F, t54144: F, t54146: F, t54148: F, t54150: F, t54154: F) -> (F, F) {
    let t55480 = 7.0 / 144.0 * t54113;
    let t55482 = 7.0 / 144.0 * t54117;
    let t55486 = 119.0 / 1728.0 * t54126;
    let t55487 = 7.0 / 288.0 * t54128;
    let t55489 = t54107 / 48.0 - t54109 / 24.0 + t54111 / 96.0 + t55480 - t54115 / 96.0 + t55482 + t54120 / 24.0 - t54122 / 24.0 + t54124 / 96.0 + t55486 - t55487 + t54130 / 48.0;
    let t55491 = 7.0 / 72.0 * t54135;
    let t55500 = 7.0 / 72.0 * t54152;
    let t55502 = t54133 / 8.0 - t55491 + t54137 / 128.0 + 3.0 / 128.0 * t54139 - 7.0 / 144.0 * t51252 + t54142 / 48.0 - t54144 / 192.0 - t54146 / 48.0 + t54148 / 24.0 - t54150 / 48.0 + t55500 - t54154 / 192.0;
    (t55489, t55502)
}
