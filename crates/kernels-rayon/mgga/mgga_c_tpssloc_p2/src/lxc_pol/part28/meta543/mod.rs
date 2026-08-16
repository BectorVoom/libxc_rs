//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta543 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1808;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1809;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta543(t213: f64, t6589: f64, t9223: f64, t6593: f64, t23062: f64, t23066: f64, t22715: f64, t229: f64, t805: f64, t1891: f64, t192: f64, t22690: f64, t80881: f64, t841: f64, t244: f64, t6546: f64, t2606: f64, t1878: f64, t845: f64, t2230: f64, t23076: f64, t23080: f64, t200: f64, t23075: f64, t598: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t81933, t81934, t81936, t81942, t81943, t81954) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1808(t213, t6589, t9223, t6593, t23062, t23066, t22715, t229, t805, t1891, t192, t22690, t80881, t841);
        let (t81956, t81957, t81959, t81962, t81964, t81968) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1809(t244, t6546, t2606, t1878, t845, t2230, t23076, t213, t23080, t200, t23075, t598);
    (t81933, t81934, t81936, t81942, t81943, t81954, t81956, t81957, t81959, t81962, t81964, t81968)
}
