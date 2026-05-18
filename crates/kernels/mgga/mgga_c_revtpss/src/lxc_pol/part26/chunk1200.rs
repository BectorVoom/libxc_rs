//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1200/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1200<F: Float>(t94386: F, t96220: F, t94391: F, t1358: F, t212: F, t26333: F, t689: F, t2097: F, t9646: F, t9648: F, t1444: F, t25921: F, t26351: F, t4131: F, t7295: F, t7296: F, t7506: F, t96188: F, t96193: F, t96195: F, t96197: F, t96206: F, t96210: F, t96211: F, t96218: F) -> (F, F) {
    let t96221 = t96220 * t94386;
    let t96222 = t94391 * t96221;
    let t96226 = t689 * t212 * t26333 * t1358;
    let t96230 = F::new(0.19637199382202157274e-3) * t9646 * t2097 * t9648;
    let t96231 = F::new(0.86736281882051994623e-1) * t96188 - F::new(0.43368140941025997312e-1) * t96193 + F::new(0.77108554593144223218e-1) * t96195 + F::new(0.21951497276451705329e-1) * t96197 + F::new(0.26020884564615598386e1) * t7295 * t7296 * t26333 * t1444 + t96206 + F::new(0.26020884564615598386e1) * t25921 * t26351 - t96210 - F::new(0.28912093960683998208e-1) * t96211 + F::new(0.26020884564615598386e1) * t7295 * t7296 * t7506 * t4131 - t96218 + F::new(0.68549505033305214441e-2) * t96222 - F::new(0.16463622957338778996e-1) * t96226 + t96230;
    (t96221, t96231)
}
