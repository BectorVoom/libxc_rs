//! MGGA_C_REVTPSS lxc pol kernel — _part41_v4rho3tau_4 meta424 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1482;
use chunk1::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1483;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_meta424(t10227: f64, t96: f64, t10199: f64, t2175: f64, t2289: f64, t8264: f64, t31377: f64, t571: f64, t1464: f64, t8372: f64, t31027: f64, t31271: f64, t116929: f64, t8358: f64, t31032: f64, t31280: f64, t46089: f64, t655: f64, t31288: f64, t116926: f64, t8355: f64, t31264: f64, t31277: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t116946, t116968, t116969, t117369, t117374, t117450) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1482(t10227, t96, t10199, t2175, t2289, t8264, t31377, t571, t1464, t8372, t31027, t31271);
        let (t117457, t117460, t117462, t117470, t117473, t117482) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1483(t116929, t8358, t31032, t31280, t46089, t655, t31288, t116926, t8355, t31027, t31264, t31277);
    (t116946, t116968, t116969, t117369, t117374, t117450, t117457, t117460, t117462, t117470, t117473, t117482)
}
