//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta270 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1042;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta270(t2476: f64, t5966: f64, t236: f64, t807: f64, t5819: f64, t633: f64, t637: f64, t221: f64, t2675: f64, t5962: f64, t2674: f64, t243: f64, t6016: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t18352, t18353, t18354, t18367, t18379, t18402, t18403, t18408) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1042(t2476, t5966, t236, t807, t5819, t633, t637, t221, t2675, t5962, t2674, t243, t6016);
    (t18352, t18353, t18354, t18367, t18379, t18402, t18403, t18408)
}
