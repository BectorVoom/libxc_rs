//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta624 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2032;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta624(t86590: f64, t25: f64, t40772: f64, t1408: f64, t2752: f64, t2: f64, t193: f64, t201: f64, t7540: f64, t870: f64, t25353: f64, t25213: f64, t6547: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t86591, t86716, t86721, t86730, t86736, t86753, t86836, t86843) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2032(t86590, t25, t40772, t1408, t2752, t2, t193, t201, t7540, t870, t25353, t25213, t6547);
    (t86591, t86716, t86721, t86730, t86736, t86753, t86836, t86843)
}
