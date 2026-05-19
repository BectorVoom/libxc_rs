//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 1023/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk1023<F: Float>(t1398: F, t30298: F, t1383: F, t1375: F, t30290: F, t30238: F, t1471: F, t30233: F, t3661: F, t457: F, t1186: F, t158: F, t165: F, t173: F, t25495: F) -> F {
    let t30803 = t1398 * t30298;
    let t30806 = t1383 * t30298;
    let t30809 = t1375 * t30290;
    let t30812 = t1375 * t30238;
    let t30815 = t1471 * t30233;
    let t30818 = t1383 * t30238;
    let t30821 = t3661 * t30233;
    let t30824 = t1398 * t30238;
    let t30827 = t457 * t30233;
    let t30830 = t1186 * t30290;
    let t30833 = t1375 * t30298;
    let t30836 = F::cast_from(0.79249999999999999999e-2_f64) * t25495 + F::new(0.30247875e-4) * t173 * t30803 + F::new(0.4755e-2) * t165 * t30806 + F::new(0.403305e-4) * t173 * t30809 - F::new(0.3513e-2) * t158 * t30812 + F::cast_from(0.78066666666666666667e-3_f64) * t158 * t30815 + F::new(0.7925e-3) * t165 * t30818 - F::cast_from(0.17611111111111111111e-3_f64) * t165 * t30821 + F::new(0.50413125e-5) * t173 * t30824 + F::cast_from(0.22405833333333333333e-5_f64) * t173 * t30827 + F::new(0.317e-2) * t165 * t30830 - F::new(0.21078e-1) * t158 * t30833;
    t30836
}
