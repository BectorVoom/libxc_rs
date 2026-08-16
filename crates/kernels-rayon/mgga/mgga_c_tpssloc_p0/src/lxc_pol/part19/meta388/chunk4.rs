//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1461/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1461(t44792: f64, t44793: f64, t44795: f64, t44796: f64, t1174: f64, t11765: f64, t135: f64, t43763: f64, t44620: f64, t3551: f64, t698: f64, t11545: f64, t43791: f64) -> (f64, f64, f64, f64, f64) {
    let t44798 = t44792 + t44793 + t44795 + t44796;
    let t44803 = t1174 * t135 * t11765;
    let t44805 = t44620 * t43763;
    let t44811 = t1174 * t698 * t3551;
    let t44817 = t11545 * t43791;
    (t44798, t44803, t44805, t44811, t44817)
}
