//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1374/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1374(t1070: f64, t11094: f64, t193: f64, t3209: f64, t3213: f64, t336: f64, t41804: f64, t41813: f64, t42276: f64, t42280: f64, t42283: f64, t42663: f64, t42665: f64, t42667: f64, t42669: f64, t42674: f64, t42678: f64, t43447: f64, t43622: f64, t4700: f64) -> f64 {
    let t43627 = -t42276 - t42280 - t42283 + 12.0_f64 * t4700 * t3213 * t11094 * t3209 + t193 * t336 * (t43447 + t43622) * t1070 + t42663 - t42665 + t41804 - t42667 + t42669 - t42674 - t41813 + t42678;
    t43627
}
