//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1140/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1140<F: Float>(t2482: F, t27: F, t7262: F, t4021: F, t25273: F, t533: F, t816: F, t540: F, t7021: F, t1372: F, t3961: F, t7252: F) -> (F, F, F, F, F, F) {
    let t25997 = t2482 * t7262 * t27;
    let t25998 = t25997 * t4021;
    let t26002 = t25273 * t533 * t816;
    let t26003 = F::new(35.0) / F::new(432.0) * t26002;
    let t26004 = t7021 * t540;
    let t26005 = t26004 * t1372;
    let t26006 = F::new(7.0) / F::new(72.0) * t26005;
    let t26007 = t7252 * t3961;
    (t25997, t25998, t26003, t26004, t26006, t26007)
}
