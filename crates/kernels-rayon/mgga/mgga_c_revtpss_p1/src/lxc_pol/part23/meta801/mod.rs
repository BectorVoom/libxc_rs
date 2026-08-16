//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta801 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2628;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2629;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta801(t18657: f64, t212: f64, t689: f64, t780: f64, t252: f64, t2769: f64, t2782: f64, t6071: f64, t886: f64, t4500: f64, t51421: f64, t14495: f64, t14567: f64, t18616: f64, t2798: f64, t686: f64, t72: f64, t61532: f64, t836: f64, t39597: f64, t6022: f64, t10529: f64, t10952: f64, t18525: f64, t2482: f64, t5977: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t62549, t62572, t62577, t62583) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2628(t18657, t212, t689, t780, t252, t2769, t2782, t6071, t886, t4500, t51421, t14495, t14567);
        let (t62587, t62591, t62595, t62601) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2629(t18616, t2798, t686, t72, t61532, t836, t2782, t39597, t6022, t10529, t10952, t18525, t2482, t5977);
    (t62549, t62572, t62577, t62583, t62587, t62591, t62595, t62601)
}
