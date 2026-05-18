//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1075/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1075<F: Float>(t212: F, t7506: F, t1358: F, t689: F, t2097: F, t785: F, t2439: F, t2435: F, t7493: F, t26069: F, t26277: F, t26072: F, t7515: F) -> (F, F, F, F, F, F, F, F, F) {
    let t26354 = t212 * t7506;
    let t26355 = t26354 * t1358;
    let t26356 = t689 * t26355;
    let t26358 = t785 * t2097;
    let t26359 = t26358 * t1358;
    let t26361 = F::new(0.65049603595885220126e-3) * t2439 * t26359;
    let t26363 = F::new(0.73171657588172351096e-2) * t2435 * t7493;
    let t26365 = F::new(0.22849835011101738147e-2) * t26069 * t26277;
    let t26366 = t26072 * t7515;
    (t26354, t26355, t26356, t26358, t26359, t26361, t26363, t26365, t26366)
}
