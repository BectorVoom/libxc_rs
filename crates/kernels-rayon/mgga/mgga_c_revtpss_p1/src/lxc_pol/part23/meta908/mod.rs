//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta908 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2916;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2917;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2918;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta908(t23535: f64, t2880: f64, t918: f64, t51914: f64, t51915: f64, t63240: f64, t63242: f64, t77663: f64, t77667: f64, t77670: f64, t77672: f64, t77674: f64, t77676: f64, t2897: f64, t23540: f64, t41401: f64, t18979: f64, t4606: f64, t15113: f64, t6120: f64, t18950: f64, t4598: f64, t41382: f64, t15107: f64, t15110: f64, t41246: f64, t77499: f64, t77503: f64, t77505: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t77679, t77681) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2916(t23535, t2880, t918, t51914, t51915, t63240, t63242, t77663, t77667, t77670, t77672, t77674, t77676);
        let (t77683, t77686, t77688, t77690, t77692, t77695) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2917(t23535, t2897, t918, t23540, t41401, t18979, t4606, t15113, t6120, t18950, t4598, t41382);
        let (t77698, t77700, t77705) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2918(t6120, t918, t15107, t15110, t41246, t77499, t77503, t77505, t77683, t77686, t77688, t77690, t77692, t77695);
    (t77679, t77681, t77683, t77686, t77688, t77690, t77692, t77695, t77698, t77700, t77705)
}
