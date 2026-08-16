//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1482/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1482(t1858: f64, t8843: f64, t2174: f64, t7945: f64, t34175: f64, t580: f64, t2169: f64, t7961: f64, t34194: f64, t576: f64, t117418: f64, t117430: f64, t1396: f64, t1404: f64, t2105: f64, t27241: f64, t27908: f64, t34176: f64, t5364: f64, t5381: f64, t7223: f64, t7240: f64, t7426: f64, t7946: f64, t8111: f64, t8119: f64, t8844: f64, t8852: f64) -> f64 {
    let t125065 = t8843 * t1858;
    let t125067 = t7945 * t2174;
    let t125068 = t34175 * t580;
    let t125069 = t2169 * t7961;
    let t125071 = t576 * t34194;
    let t125073 = t1396 * t34194 + t1404 * t34176 + t2105 * t27908 + t2174 * t27241 + t5364 * t8852 + t5381 * t8844 + t7223 * t8119 + t7240 * t8111 + t7426 * t7946 + t117418 + t117430 + t125065 + t125067 + t125068 + t125069 + t125071;
    t125073
}
