//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1228/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1228<F: Float>(t40090: F, t40102: F, t40109: F, t38074: F, t38076: F, t38079: F, t38622: F, t40092: F, t40095: F, t40098: F, t40100: F, t40107: F) -> F {
    let t41689 = F::cast_from(0.11177905488750909899e1_f64) * t40090;
    let t41694 = F::cast_from(0.39029762157531132074e-1_f64) * t40102;
    let t41699 = F::cast_from(0.84755945902752848174e0_f64) * t40109;
    let t41700 = t41689 + F::cast_from(0.20803732176130244552e1_f64) * t40092 + F::cast_from(0.2600466522016280569e0_f64) * t40095 + F::cast_from(0.87327386630866483588e-2_f64) * t40098 - F::cast_from(0.26198215989259945076e-1_f64) * t40100 + t41694 - t38622 + F::cast_from(0.69345773920434148506e0_f64) * t38074 + F::cast_from(0.13869154784086829701e1_f64) * t38076 + F::cast_from(0.23115257973478049502e0_f64) * t38079 + F::cast_from(0.58544643236296698113e-1_f64) * t40107 + t41699;
    t41700
}
