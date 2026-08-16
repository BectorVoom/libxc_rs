//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta346 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1679;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1680;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta346(t11249: f64, t11631: f64, t11248: f64, t1042: f64, t2251: f64, t999: f64, t4801: f64, t1041: f64, t1047: f64, t1063: f64, t11233: f64, t11246: f64, t11252: f64, t11256: f64, t11259: f64, t11264: f64, t11268: f64, t11271: f64, t11274: f64, t11277: f64, t11281: f64, t11286: f64, t11623: f64, t11630: f64, t3124: f64, t3127: f64, t3136: f64, t3157: f64, t3164: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t11632, t11633, t11634, t11637) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1679(t11249, t11631, t11248, t1042, t2251, t999);
        let (t11638, t11639, t11642) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1680(t11637, t4801, t1042, t1041, t1047, t1063, t11233, t11246, t11252, t11256, t11259, t11264, t11268, t11271, t11274, t11277, t11281, t11286, t11623, t11630, t11634, t3124, t3127, t3136, t3157, t3164);
    (t11632, t11633, t11634, t11637, t11638, t11639, t11642)
}
