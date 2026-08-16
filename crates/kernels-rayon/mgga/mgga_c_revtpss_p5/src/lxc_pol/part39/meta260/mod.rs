//! MGGA_C_REVTPSS lxc pol kernel — _part39_v4rho3tau_2 meta260 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk971;
use chunk1::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk972;
use chunk2::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk973;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_meta260(t508: f64, t8362: f64, t569: f64, t1911: f64, t2178: f64, t1312: f64, t2179: f64, t2181: f64, t4248: f64, t651: f64, t7732: f64, t7889: f64, t8353: f64, t3: f64, t1518: f64, t8295: f64, t117: f64, t1916: f64, t1918: f64, t2187: f64, t2189: f64, t572: f64, t573: f64, param_d: f64, t587: f64, t65: f64, t143: f64, t2580: f64, t130: f64, t2566: f64, t700: f64, t2584: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t8363, t8367, t8369, t8372) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk971(t508, t8362, t569, t1911, t2178, t1312, t2179, t2181, t4248, t651, t7732, t7889, t8353);
        let (t8373, t8377, t8383, t8386, t8389) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk972(t3, t8372, t1518, t8295, t117, t8362, t1916, t1918, t2187, t2189, t572, t573, param_d);
        let (t8779, t9275, t9278) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk973(t587, t65, t143, t2580, t130, t2566, t700, t2584);
    (t8363, t8367, t8369, t8372, t8373, t8377, t8383, t8386, t8389, t8779, t9275, t9278)
}
