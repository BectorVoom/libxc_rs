//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1116/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1116<F: Float>(t10667: F, t42318: F, t3262: F, t3472: F, t40523: F, t38303: F, t38308: F, t39116: F, t39117: F, t39121: F, t40659: F, t40672: F, t42302: F, t42304: F, t42307: F, t42310: F, t42313: F, t42316: F) -> (F, F, F) {
    let t42320 = 3.0 / 2.0 * t10667 * t42318;
    let t42326 = 15.0 / 16.0 * t3262 * t3472 * t40523;
    let t42327 = t42302 - t42304 - t39116 + 0.68400385060046895e-6 * t40659 + t42307 + t42310 + t42313 + t42316 - t42320 - 0.7044137609176975208e-2 * t40672 - t39117 - 0.2881692658299671676e-2 * t38303 + 0.72042316457491791901e-3 * t38308 + t39121 + t42326;
    (t42320, t42326, t42327)
}
