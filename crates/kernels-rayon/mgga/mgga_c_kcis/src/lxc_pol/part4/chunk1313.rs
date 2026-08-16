//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1313/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1313(t110: f64, t1939: f64, t493: f64, t1930: f64, t3974: f64, t1369: f64, t2469: f64, t5714: f64, t1368: f64, t1593: f64, t5727: f64, t12133: f64, t1933: f64) -> (f64, f64, f64, f64, f64) {
    let t16841 = t110 * t1939;
    let t16842 = t493 * t16841;
    let t16845 = t1930 * t3974 / 54.0_f64;
    let t16848 = t2469 * t1369;
    let t16849 = t16848 * t5714;
    let t16850 = t1368 * t16849;
    let t16852 = t1593 * t5727;
    let t16854 = t1368 * t16852 / 72.0_f64;
    let t16857 = t12133 * t1933;
    (t16842, t16845, t16850, t16854, t16857)
}
