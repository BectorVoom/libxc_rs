//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1184/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1184<F: Float>(t12049: F, t12053: F, t12055: F, t12058: F, t12061: F, t12063: F, t12228: F, t12081: F, t12084: F, t12087: F, t11198: F, t11202: F, t11205: F, t11324: F, t11328: F, t11330: F) -> F {
    let t41126 = t12049 / F::new(2.0);
    let t41127 = t12053 / F::new(2.0);
    let t41128 = F::new(2.0) * t12055;
    let t41129 = F::new(3.0) / F::new(2.0) * t12058;
    let t41130 = F::new(5.0) / F::new(8.0) * t12061;
    let t41131 = F::new(2.0) * t12063;
    let t41132 = F::new(2.0) * t12228;
    let t41133 = F::new(3.0) / F::new(2.0) * t12081;
    let t41134 = t12084 / F::new(2.0);
    let t41135 = t12087 / F::new(2.0);
    let t41136 = -t41126 + t41127 + t41128 + t11198 + t11202 + t41129 + t41130 - t11205 + t11324 + t41131 + t41132 - t11328 + t41133 + t11330 - t41134 + t41135;
    t41136
}
