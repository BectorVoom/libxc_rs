//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1232/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1232<F: Float>(t40201: F, t40215: F, t40217: F, t40220: F, t40222: F, t40228: F, t40197: F, t40204: F, t40207: F, t40210: F, t40213: F, t40224: F) -> F {
    let t41743 = F::new(0.19043987679069580389e-1) * t40201;
    let t41748 = F::new(0.19514881078765566037e-1) * t40215;
    let t41749 = F::new(0.21951497276451705328e-1) * t40217;
    let t41750 = F::new(0.45022119329691164871e0) * t40220;
    let t41751 = F::new(0.46230515946956099004e0) * t40222;
    let t41753 = F::new(0.32524801797942610062e-3) * t40228;
    let t41754 = -F::new(0.65854491829355115984e0) * t40197 - t41743 + F::new(0.5200933044032561138e0) * t40204 - F::new(0.5200933044032561138e0) * t40207 + F::new(0.21951497276451705328e0) * t40210 - F::new(0.20803732176130244552e1) * t40213 - t41748 - t41749 + t41750 - t41751 - F::new(0.87327386630866483588e-2) * t40224 + t41753;
    t41754
}
