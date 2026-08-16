//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta896 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2855;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta896(t23121: f64, t40188: f64, t40121: f64, t40132: f64, t40139: f64, t40088: f64, t40099: f64, t40103: f64, t40115: f64, t40131: f64, t40137: f64, t50048: f64, t76986: f64, t76987: f64, t76988: f64, t76991: f64, t76992: f64, t76995: f64) -> (f64, f64, f64, f64, f64) {
        let (t76997, t76998, t76999, t77000, t77001) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2855(t23121, t40188, t40121, t40132, t40139, t40088, t40099, t40103, t40115, t40131, t40137, t50048, t76986, t76987, t76988, t76991, t76992, t76995);
    (t76997, t76998, t76999, t77000, t77001)
}
