//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1029/1507 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1029<F: Float>(t1445: F, t3899: F, t689: F, t10115: F, t562: F, t2435: F, t3903: F, t3895: F, t2439: F, t1420: F, t2453: F, t3908: F) -> (F, F, F, F, F) {
    let t10153 = t3899 * t1445;
    let t10154 = t689 * t10153;
    let t10157 = F::cast_from(0.11044544084478153697e-3_f64) * t10115 * t562;
    let t10160 = t2435 * t3903;
    let t10162 = t3895 * t1445;
    let t10163 = t2439 * t10162;
    let t10165 = t2453 * t1420;
    let t10166 = t10165 * t3908;
    (t10154, t10157, t10160, t10163, t10166)
}
