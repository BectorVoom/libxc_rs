//! MGGA_C_REVTPSS lxc pol kernel — _part41_v4rho3tau_4 meta424 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1482;
use chunk1::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1483;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_meta424<F: Float>(t10227: F, t96: F, t10199: F, t2175: F, t2289: F, t8264: F, t31377: F, t571: F, t1464: F, t8372: F, t31027: F, t31271: F, t116929: F, t8358: F, t31032: F, t31280: F, t46089: F, t655: F, t31288: F, t116926: F, t8355: F, t31264: F, t31277: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t116946, t116968, t116969, t117369, t117374, t117450) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1482::<F>(t10227, t96, t10199, t2175, t2289, t8264, t31377, t571, t1464, t8372, t31027, t31271);
        let (t117457, t117460, t117462, t117470, t117473, t117482) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1483::<F>(t116929, t8358, t31032, t31280, t46089, t655, t31288, t116926, t8355, t31027, t31264, t31277);
    (t116946, t116968, t116969, t117369, t117374, t117450, t117457, t117460, t117462, t117470, t117473, t117482)
}
