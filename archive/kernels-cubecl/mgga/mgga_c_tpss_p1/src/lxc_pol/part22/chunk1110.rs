//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1110/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1110<F: Float>(t12241: F, t408: F, t1505: F, t2861: F, t2864: F, t2913: F, t4104: F, t1042: F, t2911: F, t2905: F, t4108: F, t1518: F, t9495: F) -> (F, F, F, F, F) {
    let t12243 = F::cast_from(0.621814e-1_f64) * t12241 * t408;
    let t12244 = t1505 * t2861;
    let t12246 = F::cast_from(2.0_f64) * t12244 * t2864;
    let t12247 = t4104 * t2913;
    let t12248 = t12247 * t1042;
    let t12250 = F::cast_from(0.32163958997385070134e2_f64) * t2911 * t12248;
    let t12251 = t4108 * t2905;
    let t12253 = F::cast_from(0.16081979498692535067e2_f64) * t2911 * t12251;
    let t12254 = t1518 * t9495;
    (t12243, t12246, t12250, t12253, t12254)
}
