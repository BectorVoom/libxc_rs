//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta498 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1929;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1930;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta498(t10996: f64, t20234: f64, t974: f64, t1616: f64, t5685: f64, t3071: f64, t5677: f64, t10408: f64, t1539: f64, t5867: f64, t21118: f64, t248: f64, t3062: f64, t21238: f64, t942: f64, t951: f64, t959: f64, t21093: f64, t21097: f64, t21099: f64, t21103: f64, t21105: f64, t21107: f64, t21365: f64, t21367: f64, t21369: f64, t21375: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t21561, t21562, t21565, t21566, t21569, t21570, t21573, t21574, t21580) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1929(t10996, t20234, t974, t1616, t5685, t3071, t5677, t10408, t1539, t5867, t21118, t248, t3062);
        let (t21589, t21591, t21592) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1930(t21238, t942, t951, t959, t21093, t21097, t21099, t21103, t21105, t21107, t21365, t21367, t21369, t21375);
    (t21561, t21562, t21565, t21566, t21569, t21570, t21573, t21574, t21580, t21589, t21591, t21592)
}
