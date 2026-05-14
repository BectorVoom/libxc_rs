//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1083/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1083<F: Float>(t40090: F, t40102: F, t40109: F, t38074: F, t38076: F, t38079: F, t38622: F, t40092: F, t40095: F, t40098: F, t40100: F, t40107: F, t40131: F, t40137: F, t38088: F, t38093: F, t40114: F, t40117: F, t40120: F, t40123: F, t40128: F, t40134: F, t40139: F, t40142: F) -> (F, F) {
    let t41689 = 0.11177905488750909899e1 * t40090;
    let t41694 = 0.39029762157531132074e-1 * t40102;
    let t41699 = 0.84755945902752848174e0 * t40109;
    let t41700 = t41689 + 0.20803732176130244552e1 * t40092 + 0.2600466522016280569e0 * t40095 + 0.87327386630866483588e-2 * t40098 - 0.26198215989259945076e-1 * t40100 + t41694 - t38622 + 0.69345773920434148506e0 * t38074 + 0.13869154784086829701e1 * t38076 + 0.23115257973478049502e0 * t38079 + 0.58544643236296698113e-1 * t40107 + t41699;
    let t41709 = 0.18629842481251516498e0 * t40131;
    let t41711 = 0.84755945902752848174e0 * t40137;
    let t41714 = -0.87327386630866483588e-2 * t40114 - 0.13099107994629972538e-1 * t40117 - 0.13099107994629972538e-1 * t40120 - 0.52396431978519890152e-1 * t40123 - 0.46574606203128791246e-1 * t38088 - 0.46574606203128791246e-1 * t38093 - 0.43663693315433241794e-2 * t40128 + t41709 + 0.87327386630866483588e-2 * t40134 - t41711 - 0.26198215989259945076e-1 * t40139 - 0.26198215989259945076e-1 * t40142;
    (t41700, t41714)
}
