//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta600 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1845;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta600(t26653: f64, t814: f64, t87520: f64, t87522: f64, t87533: f64, t87535: f64, t87544: f64, t87546: f64, t87197: f64, t87205: f64, t87211: f64, t87233: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t92546, t92551, t92556, t92560, t92561, t92564, t92565, t92578, t92580, t92582, t92590) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1845(t26653, t814, t87520, t87522, t87533, t87535, t87544, t87546, t87197, t87205, t87211, t87233);
    (t92546, t92551, t92556, t92560, t92561, t92564, t92565, t92578, t92580, t92582, t92590)
}
