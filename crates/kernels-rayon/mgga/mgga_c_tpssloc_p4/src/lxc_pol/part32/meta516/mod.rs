//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta516 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1847;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta516(t225: f64, t7723: f64, t2015: f64, t5353: f64, t3887: f64, t22897: f64, t5336: f64, t1992: f64, t22751: f64, t7733: f64, t1799: f64, t22881: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t26366, t26371, t26378, t26379, t26381, t26384) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1847(t225, t7723, t2015, t5353, t3887, t22897, t5336, t1992, t22751, t7733, t1799, t22881);
    (t26366, t26371, t26378, t26379, t26381, t26384)
}
