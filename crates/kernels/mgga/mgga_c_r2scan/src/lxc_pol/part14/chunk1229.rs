//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1229/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1229<F: Float>(t40131: F, t40137: F, t38088: F, t38093: F, t40114: F, t40117: F, t40120: F, t40123: F, t40128: F, t40134: F, t40139: F, t40142: F) -> F {
    let t41709 = F::new(0.18629842481251516498e0) * t40131;
    let t41711 = F::new(0.84755945902752848174e0) * t40137;
    let t41714 = -F::new(0.87327386630866483588e-2) * t40114 - F::new(0.13099107994629972538e-1) * t40117 - F::new(0.13099107994629972538e-1) * t40120 - F::new(0.52396431978519890152e-1) * t40123 - F::new(0.46574606203128791246e-1) * t38088 - F::new(0.46574606203128791246e-1) * t38093 - F::new(0.43663693315433241794e-2) * t40128 + t41709 + F::new(0.87327386630866483588e-2) * t40134 - t41711 - F::new(0.26198215989259945076e-1) * t40139 - F::new(0.26198215989259945076e-1) * t40142;
    t41714
}
