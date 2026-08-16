//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta273 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1046;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta273(t125: f64, t5966: f64, t2652: f64, t5993: f64, t6030: f64, t10858: f64, t6024: f64, t2741: f64, t6019: f64, t10811: f64, t6037: f64, t221: f64, t2485: f64, t5978: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t18469, t18475, t18485, t18487, t18491, t18518, t18531) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1046(t125, t5966, t2652, t5993, t6030, t10858, t6024, t2741, t6019, t10811, t6037, t221, t2485, t5978);
    (t18469, t18475, t18485, t18487, t18491, t18518, t18531)
}
