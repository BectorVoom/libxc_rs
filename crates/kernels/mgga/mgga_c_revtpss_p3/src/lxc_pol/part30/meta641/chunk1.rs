//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2229/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2229<F: Float>(t15904: F, t26865: F, t13127: F, t17400: F, t26866: F, t1802: F, t3089: F, t3717: F, t13148: F, t17558: F, t17625: F, t17713: F, t17756: F, t17786: F, t29100: F, t3723: F, t7624: F, t97136: F, t97141: F, t97154: F, t97161: F, t97179: F, sigma2: F) -> (F, F, F) {
    let t104695 = t26865 * t15904;
    let t104696 = t13127 * t104695;
    let t104703 = t17400 * t26866;
    let t104706 = sigma2 * t1802;
    let t104707 = t104706 * t3089;
    let t104708 = t3717 * t104707;
    let t104715 = t13148 * t104695;
    let t104718 = -F::cast_from(0.42874018118069736972e-3_f64) * t29100 * t17786 + F::cast_from(0.42874018118069736972e-3_f64) * t104696 * t17756 + F::cast_from(0.3811023832717309953e-3_f64) * t97136 + F::cast_from(0.47637797908966374413e-3_f64) * t7624 * t17558 + F::cast_from(0.1270341277572436651e-3_f64) * t97141 - F::cast_from(0.85748036236139473944e-3_f64) * t104703 * t3723 + F::cast_from(0.45732285992607719436e-2_f64) * t104708 * t3723 + F::cast_from(0.85748036236139473944e-3_f64) * t97179 * t17625 - F::cast_from(0.3811023832717309953e-3_f64) * t97154 + F::cast_from(0.31758531939310916275e-3_f64) * t97161 + F::cast_from(0.25724410870841842183e-2_f64) * t104715 * t17713;
    (t104695, t104707, t104718)
}
