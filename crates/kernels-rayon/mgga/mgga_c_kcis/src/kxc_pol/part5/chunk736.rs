//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 736/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk736(t1335: f64, t5573: f64, t1316: f64, t1906: f64, t3901: f64, t1334: f64, t3899: f64, t3795: f64, t3905: f64, t5469: f64, t5472: f64, t5475: f64, t5479: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t5574 = t5573 * t1335;
    let t5576 = 1.0_f64 * t1316 * t5574;
    let t5577 = t1906 * t3901;
    let t5578 = t5577 * t1334;
    let t5580 = 0.16081824322151104822e2_f64 * t3899 * t5578;
    let t5586 = t3905 + 0.30902777777777777778e-2_f64 * t3795 + 0.30902777777777777778e-2_f64 * t5469 - 0.61805555555555555555e-2_f64 * t5472 + 0.18541666666666666667e-1_f64 * t5475 + 0.18541666666666666667e-1_f64 * t5479;
    (t5574, t5576, t5577, t5578, t5580, t5586)
}
