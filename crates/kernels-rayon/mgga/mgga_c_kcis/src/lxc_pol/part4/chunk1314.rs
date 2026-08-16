//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1314/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1314(t1368: f64, t16857: f64, t12159: f64, t1938: f64, t4001: f64, t613: f64, t3971: f64, t5691: f64, t16830: f64, t16833: f64, t16838: f64, t16842: f64, t16845: f64, t16850: f64, t16854: f64, t1930: f64, t3991: f64, t3995: f64, t4003: f64, t493: f64) -> f64 {
    let t16858 = t1368 * t16857;
    let t16861 = t12159 * t1938 * t4001;
    let t16862 = t613 * t16861;
    let t16866 = t5691 * t3971 / 162.0_f64;
    let t16869 = t16830 * t16833 / 72.0_f64 - t493 * t16838 / 144.0_f64 + t16842 / 432.0_f64 + t16845 - t1930 * t4003 / 18.0_f64 + 7.0_f64 / 432.0_f64 * t16850 + t16854 + t5691 * t3991 / 54.0_f64 - t16858 / 1296.0_f64 - t1368 * t16862 / 16.0_f64 - t16866 - t5691 * t3995 / 108.0_f64;
    t16869
}
