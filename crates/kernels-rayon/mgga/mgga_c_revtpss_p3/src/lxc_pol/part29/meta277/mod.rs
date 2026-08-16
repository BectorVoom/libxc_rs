//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta277 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1142;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1143;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta277(t532: f64, t8107: f64, t1450: f64, t2107: f64, t5542: f64, t118: f64, t1502: f64, t1519: f64, t1843: f64, t1911: f64, t2014: f64, t2052: f64, t2056: f64, t2089: f64, t2093: f64, t2108: f64, t4248: f64, t508: f64, t569: f64, t651: f64, t7359: f64, t7732: f64, t7898: f64, t7969: f64, t7978: f64, t7984: f64, t7988: f64, t8065: f64, t8075: f64, t8079: f64, t3: f64, t1518: f64, t7553: f64, t117: f64, t7983: f64, t1916: f64, t1918: f64, t2113: f64, t2115: f64, t572: f64, t573: f64, param_d: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t8108, t8109, t8111, t8113) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1142(t532, t8107, t1450, t2107, t5542, t118, t1502, t1519, t1843, t1911, t2014, t2052, t2056, t2089, t2093, t2108, t4248, t508, t569, t651, t7359, t7732, t7898, t7969, t7978, t7984, t7988, t8065, t8075, t8079);
        let (t8114, t8118, t8124, t8127, t8130) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1143(t3, t8113, t1518, t7553, t117, t7983, t1916, t1918, t2113, t2115, t572, t573, param_d);
    (t8108, t8109, t8111, t8113, t8114, t8118, t8124, t8127, t8130)
}
