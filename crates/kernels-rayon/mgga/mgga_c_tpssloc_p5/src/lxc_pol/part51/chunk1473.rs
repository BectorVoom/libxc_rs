//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1473/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1473(t2105: f64, t7758: f64, t2029: f64, t7945: f64, t2022: f64, t7961: f64, t116021: f64, t116026: f64, t116032: f64, t116038: f64, t116044: f64, t1396: f64, t1404: f64, t1852: f64, t1858: f64, t2023: f64, t27286: f64, t31782: f64, t31820: f64, t33628: f64, t33662: f64, t5364: f64, t7003: f64, t7240: f64, t7759: f64, t8660: f64) -> f64 {
    let t122860 = t7758 * t2105;
    let t122862 = t7945 * t2029;
    let t122864 = t2022 * t7961;
    let t122870 = t1396 * t33662 + t1404 * t33628 + t1852 * t31820 + t1858 * t31782 + t2023 * t27286 + t5364 * t8660 + t7003 * t7961 + t7240 * t7759 + t116021 + t116026 + t116032 + t116038 + t116044 + t122860 + t122862 + t122864;
    t122870
}
