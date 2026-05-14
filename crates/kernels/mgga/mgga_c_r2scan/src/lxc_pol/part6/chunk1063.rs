//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1063/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1063<F: Float>(t259: F, t8196: F, t571: F, t795: F, t910: F, t6959: F, t7031: F, t7032: F, t7051: F, t7054: F, t7107: F, t7109: F, t7111: F, t7127: F, t7157: F, t7159: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t9520 = t8196 * t259;
    let t9521 = t571 * t9520;
    let t9577 = t910 * t795;
    let t9907 = 0.32530743900905219526e-1 * t6959;
    let t9911 = 3.0 * t7031;
    let t9912 = 0.73245789224026180216e-3 * t7032;
    let t9914 = 24.0 * t7051;
    let t9915 = 24.0 * t7054;
    let t9916 = 96.0 * t7107;
    let t9917 = 60.0 * t7109;
    let t9918 = 36.0 * t7111;
    let t9921 = 0.35089341735807877242e1 * t7127;
    let t9922 = 0.17544670867903938621e1 * t7157;
    let t9923 = 0.51947577317044391276e2 * t7159;
    (t9520, t9521, t9577, t9907, t9911, t9912, t9914, t9915, t9916, t9917, t9918, t9921, t9922, t9923)
}
