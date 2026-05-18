//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 883/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk883<F: Float>(t2061: F, t785: F, t780: F, t2439: F, t2435: F, t7385: F, t212: F, t7398: F, t689: F, t25219: F, t25231: F, t25242: F) -> (F, F, F, F, F, F) {
    let t26434 = t785 * t2061;
    let t26435 = t26434 * t780;
    let t26437 = F::new(0.65049603595885220126e-3) * t2439 * t26435;
    let t26439 = F::new(0.73171657588172351096e-2) * t2435 * t7385;
    let t26446 = t212 * t7398;
    let t26447 = t26446 * t780;
    let t26448 = t689 * t26447;
    let t26450 = F::new(0.22675591804667994221e-1) * t25219;
    let t26454 = F::new(0.54208002996571016773e-3) * t25231;
    let t26457 = F::new(0.18071592998981862717e-4) * t25242;
    (t26437, t26439, t26448, t26450, t26454, t26457)
}
