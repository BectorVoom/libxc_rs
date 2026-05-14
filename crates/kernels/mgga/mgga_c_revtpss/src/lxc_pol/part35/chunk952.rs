//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 952/1093 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk952<F: Float>(t2061: F, t785: F, t780: F, t2439: F, t2435: F, t7385: F, t25219: F, t25231: F, t25242: F, t25253: F, t25275: F, t25283: F, t2062: F, t867: F, t786: F, t2470: F, t7406: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t26434 = t785 * t2061;
    let t26435 = t26434 * t780;
    let t26437 = 0.65049603595885220126e-3 * t2439 * t26435;
    let t26439 = 0.73171657588172351096e-2 * t2435 * t7385;
    let t26450 = 0.22675591804667994221e-1 * t25219;
    let t26454 = 0.54208002996571016773e-3 * t25231;
    let t26457 = 0.18071592998981862717e-4 * t25242;
    let t26462 = 0.30488190661738479625e-3 * t25253;
    let t26468 = 35.0 / 216.0 * t25275;
    let t26471 = 0.10164000561857065645e-4 * t25283;
    let t26496 = t2062 * t867;
    let t26497 = t786 * t26496;
    let t26506 = t7406 * t2470;
    (t26434, t26435, t26437, t26439, t26450, t26454, t26457, t26462, t26468, t26471, t26496, t26497, t26506)
}
