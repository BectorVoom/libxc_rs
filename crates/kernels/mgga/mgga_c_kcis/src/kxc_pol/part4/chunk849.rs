//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 849/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk849<F: Float>(t1335: F, t5573: F, t1316: F, t1906: F, t3901: F, t1334: F, t3899: F, t3795: F, t3905: F, t5469: F, t5472: F, t5475: F, t5479: F) -> (F, F, F, F, F, F) {
    let t5574 = t5573 * t1335;
    let t5576 = F::cast_from(1.0_f64) * t1316 * t5574;
    let t5577 = t1906 * t3901;
    let t5578 = t5577 * t1334;
    let t5580 = F::cast_from(0.16081824322151104822e2_f64) * t3899 * t5578;
    let t5586 = t3905 + F::cast_from(0.30902777777777777778e-2_f64) * t3795 + F::cast_from(0.30902777777777777778e-2_f64) * t5469 - F::cast_from(0.61805555555555555555e-2_f64) * t5472 + F::cast_from(0.18541666666666666667e-1_f64) * t5475 + F::cast_from(0.18541666666666666667e-1_f64) * t5479;
    (t5574, t5576, t5577, t5578, t5580, t5586)
}
