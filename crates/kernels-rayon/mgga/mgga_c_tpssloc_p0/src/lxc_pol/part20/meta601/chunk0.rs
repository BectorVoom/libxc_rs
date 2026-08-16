//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2181/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2181(t1174: f64, t1179: f64, t44633: f64, t11529: f64, t3460: f64, t3456: f64, t10469: f64, t1190: f64, t11887: f64, t42339: f64, t466: f64, t11715: f64, t42341: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t44635 = t1174 * t44633 * t1179;
    let t44638 = t1174 * t11529 * t3460;
    let t44641 = t1174 * t11529 * t3456;
    let t44690 = t1190 * t10469;
    let t44691 = t44690 * t11887;
    let t44696 = t466 * t42339;
    let t44698 = t44696 * t42341 * t11715;
    (t44635, t44638, t44641, t44690, t44691, t44696, t44698)
}
