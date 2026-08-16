//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta246 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1071;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1072;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1073;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta246(t11173: f64, t996: f64, t1096: f64, t3325: f64, t3269: f64, t3075: f64, t1079: f64, t1071: f64, t3057: f64, t3259: f64, t994: f64, t342: f64, t992: f64, t338: f64, t378: f64, t3059: f64, t999: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t11174, t11177, t11178, t11184, t11187, t11190, t11195) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1071(t11173, t996, t1096, t3325, t3269, t3075, t1079, t1071, t3057, t3259, t994, t342);
        let (t11198, t11199, t11200) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1072(t992, t338);
        let (t11201, t11202) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1073(t11200, t378, t3059, t999);
    (t11174, t11177, t11178, t11184, t11187, t11190, t11195, t11198, t11199, t11200, t11201, t11202)
}
