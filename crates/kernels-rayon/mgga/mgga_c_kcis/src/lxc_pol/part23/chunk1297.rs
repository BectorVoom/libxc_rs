//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1297/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1297(t98344: f64, t18171: f64, t28700: f64, t27583: f64, t27566: f64, t28713: f64, t1307: f64, t1616: f64, t27586: f64, t27598: f64, t28727: f64, t4440: f64, t531: f64, t6163: f64, t6183: f64, t7978: f64, t7979: f64, t95001: f64, t95004: f64, t95007: f64, t98370: f64, t98373: f64, t99004: f64) -> (f64, f64, f64, f64) {
    let t99282 = 0.30952962962962962962e-2_f64 * t98344;
    let t99291 = t18171 * t28700;
    let t99293 = 0.7722800925925925926e-4_f64 * t27583 * t99291;
    let t99301 = t28713 * t27566;
    let t99314 = 0.51485339506172839506e-4_f64 * t95001 + t99293 + 0.13901041666666666667e-2_f64 * t27583 * t99004 + 0.23168402777777777778e-3_f64 * t27583 * t4440 * t1616 * t6183 * t1307 + 0.23168402777777777778e-3_f64 * t99301 * t27586 - 0.23168402777777777778e-3_f64 * t95004 + 0.18534722222222222222e-2_f64 * t28727 * t27598 + 0.15459116753472222222e-4_f64 * t95007 + 0.23168402777777777778e-3_f64 * t7978 * t6163 * t7979 * t531 - 0.23214722222222222222e-2_f64 * t98370 - 0.77382407407407407406e-3_f64 * t98373;
    (t99282, t99291, t99301, t99314)
}
