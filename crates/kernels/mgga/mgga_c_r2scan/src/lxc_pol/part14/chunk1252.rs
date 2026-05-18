//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1252/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1252<F: Float>(t39244: F, t39251: F, t39255: F, t39247: F, t41160: F, t41162: F, t41165: F, t41168: F, t41170: F, t41173: F, t41176: F, t41179: F, t41182: F, t41185: F, t41188: F) -> F {
    let t42162 = F::new(0.1440846329149835838e-2) * t39244;
    let t42164 = F::new(0.1440846329149835838e-2) * t39251;
    let t42165 = F::new(0.1440846329149835838e-2) * t39255;
    let t42166 = -t41160 - t41162 - t41165 + t41168 + t41170 + t41173 + t42162 - F::new(0.72042316457491791901e-3) * t39247 + t42164 + t42165 + t41176 + t41179 + t41182 + t41185 - t41188;
    t42166
}
