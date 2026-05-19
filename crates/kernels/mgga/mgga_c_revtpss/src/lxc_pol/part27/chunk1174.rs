//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1174/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1174<F: Float>(t27075: F, t27088: F, t3: F, t1461: F, t2170: F, t26115: F, t26117: F, t26119: F, t26122: F, t26126: F, t26129: F, t26132: F, t4162: F, t4165: F, t573: F, t7696: F, param_d: F) -> (F, F, F, F) {
    let t27089 = t27075 + t27088;
    let t27090 = t3 * t27089;
    let t27102 = param_d * t27089;
    let t27110 = F::new(6.0) * t1461 * t7696 + F::new(6.0) * t2170 * t4162 + F::new(3.0) * t2170 * t4165 + t27102 * t573 + t26115 + t26117 + t26119 + t26122 + t26126 + t26129 + t26132;
    (t27089, t27090, t27102, t27110)
}
