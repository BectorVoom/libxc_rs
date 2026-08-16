//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta619 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2374;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2375;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta619(t10832: f64, t10845: f64, t820: f64, t823: f64, t9948: f64, t839: f64, t10639: f64, t221: f64, t2484: f64, t2485: f64, t10820: f64, t2652: f64, t10841: f64, t10878: f64, t2741: f64, t2722: f64, t853: f64, t10726: f64, t10786: f64, t2661: f64, t10943: f64, t2663: f64, t2645: f64, t2662: f64, t2749: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t40357, t40360, t40361, t40365, t40367) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2374(t10832, t10845, t820, t823, t9948, t839, t10639, t221, t2484, t2485, t10820, t2652);
        let (t40374, t40376, t40378, t40381, t40385, t40390) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2375(t10841, t10845, t10878, t2741, t2722, t853, t10726, t10786, t2661, t10943, t2663, t2645, t2662, t2749);
    (t40357, t40360, t40361, t40365, t40367, t40374, t40376, t40378, t40381, t40385, t40390)
}
