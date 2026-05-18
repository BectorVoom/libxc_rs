//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1040/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1040<F: Float>(t12689: F, t459: F, t1294: F, t3790: F, t3737: F, t1284: F, t3552: F, t1204: F, t3766: F, t3153: F, t3588: F, t5480: F) -> (F, F, F, F, F, F) {
    let t12690 = t12689 * t459;
    let t12695 = t1294 * t3790;
    let t12696 = t3737 * t12695;
    let t12699 = t3552 * t1284;
    let t12702 = t1204 * t3766;
    let t12705 = t3588 * t3153;
    let t12706 = t12705 * t5480;
    (t12690, t12696, t12699, t12702, t12705, t12706)
}
