//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1297/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1297<F: Float>(t98344: F, t18171: F, t28700: F, t27583: F, t27566: F, t28713: F, t1307: F, t1616: F, t27586: F, t27598: F, t28727: F, t4440: F, t531: F, t6163: F, t6183: F, t7978: F, t7979: F, t95001: F, t95004: F, t95007: F, t98370: F, t98373: F, t99004: F) -> (F, F, F, F) {
    let t99282 = F::new(0.30952962962962962962e-2) * t98344;
    let t99291 = t18171 * t28700;
    let t99293 = F::new(0.7722800925925925926e-4) * t27583 * t99291;
    let t99301 = t28713 * t27566;
    let t99314 = F::new(0.51485339506172839506e-4) * t95001 + t99293 + F::new(0.13901041666666666667e-2) * t27583 * t99004 + F::new(0.23168402777777777778e-3) * t27583 * t4440 * t1616 * t6183 * t1307 + F::new(0.23168402777777777778e-3) * t99301 * t27586 - F::new(0.23168402777777777778e-3) * t95004 + F::new(0.18534722222222222222e-2) * t28727 * t27598 + F::new(0.15459116753472222222e-4) * t95007 + F::new(0.23168402777777777778e-3) * t7978 * t6163 * t7979 * t531 - F::new(0.23214722222222222222e-2) * t98370 - F::new(0.77382407407407407406e-3) * t98373;
    (t99282, t99291, t99301, t99314)
}
