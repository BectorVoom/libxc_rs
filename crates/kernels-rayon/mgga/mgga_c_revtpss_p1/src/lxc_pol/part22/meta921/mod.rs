//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta921 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3142;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3143;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta921(t1716: f64, t9292: f64, t12256: f64, t3617: f64, t3362: f64, t482: f64, t12268: f64, t1263: f64, t12230: f64, t5104: f64, t3555: f64, t488: f64, t17807: f64, t460: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let t56236 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3142(t1716, t9292);
        let (t56246, t56250, t56254, t56265, t56294, t56303) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3143(t12256, t3617, t3362, t482, t12268, t1263, t12230, t5104, t3555, t488, t17807, t460);
    (t56236, t56246, t56250, t56254, t56265, t56294, t56303)
}
