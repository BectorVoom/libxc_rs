//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1724/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1724<F: Float>(t2061: F, t785: F, t780: F, t2439: F, t2435: F, t7385: F, t2828: F, t7071: F, t212: F, t7398: F, t689: F, t25219: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t26434 = t785 * t2061;
    let t26435 = t26434 * t780;
    let t26437 = F::cast_from(0.65049603595885220126e-3_f64) * t2439 * t26435;
    let t26439 = F::cast_from(0.73171657588172351096e-2_f64) * t2435 * t7385;
    let t26440 = t2061 * t2828;
    let t26441 = t7071 * t26440;
    let t26446 = t212 * t7398;
    let t26447 = t26446 * t780;
    let t26448 = t689 * t26447;
    let t26450 = F::cast_from(0.22675591804667994221e-1_f64) * t25219;
    (t26434, t26435, t26437, t26439, t26440, t26441, t26446, t26447, t26448, t26450)
}
