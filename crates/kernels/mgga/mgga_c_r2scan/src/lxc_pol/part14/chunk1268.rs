//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1268/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1268<F: Float>(t10610: F, t3472: F, t40487: F, t1115: F, t2526: F, t3270: F, t10667: F, t3262: F, t40523: F, t38303: F, t38308: F, t39116: F, t39117: F, t39121: F, t40659: F, t40672: F, t42302: F, t42304: F, t42307: F, t42310: F, t42313: F) -> (F, F, F, F) {
    let t42316 = F::new(15.0) / F::new(8.0) * t10610 * t3472 * t40487;
    let t42318 = t3270 * t1115 * t2526;
    let t42320 = F::new(3.0) / F::new(2.0) * t10667 * t42318;
    let t42326 = F::new(15.0) / F::new(16.0) * t3262 * t3472 * t40523;
    let t42327 = t42302 - t42304 - t39116 + F::cast_from(0.68400385060046895e-6_f64) * t40659 + t42307 + t42310 + t42313 + t42316 - t42320 - F::cast_from(0.7044137609176975208e-2_f64) * t40672 - t39117 - F::cast_from(0.2881692658299671676e-2_f64) * t38303 + F::cast_from(0.72042316457491791901e-3_f64) * t38308 + t39121 + t42326;
    (t42316, t42320, t42326, t42327)
}
