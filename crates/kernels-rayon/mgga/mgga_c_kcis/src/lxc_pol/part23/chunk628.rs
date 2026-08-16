//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 628/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk628(t1506: f64, t6048: f64, t3795: f64, t4318: f64, t5469: f64, t5472: f64, t5475: f64, t5479: f64, t1563: f64, t2072: f64, t1571: f64, t2080: f64) -> (f64, f64, f64, f64) {
    let t6049 = t1506 * t6048;
    let t6072 = t4318 + 0.57077777777777777777e-2_f64 * t3795 + 0.57077777777777777777e-2_f64 * t5469 - 0.11415555555555555555e-1_f64 * t5472 + 0.34246666666666666666e-1_f64 * t5475 + 0.34246666666666666666e-1_f64 * t5479;
    let t6075 = t2072 * t1563;
    let t6080 = t2080 * t1571;
    (t6049, t6072, t6075, t6080)
}
