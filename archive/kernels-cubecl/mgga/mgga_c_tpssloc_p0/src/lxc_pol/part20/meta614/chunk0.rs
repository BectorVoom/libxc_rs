//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2204/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2204<F: Float>(t1454: F, t2585: F, t2281: F, t4044: F, t12758: F, t626: F, t12761: F, t12754: F, t4068: F, t12809: F, t92: F, t9384: F) -> (F, F, F, F, F, F, F, F) {
    let t45656 = t2585 * t1454;
    let t45658 = t2281 * t4044;
    let t45659 = F::cast_from(22.0_f64) / F::cast_from(3.0_f64) * t45658;
    let t45660 = t626 * t12758;
    let t45662 = t626 * t12761;
    let t45676 = t626 * t12754;
    let t45688 = t2281 * t4068;
    let t45689 = F::cast_from(11.0_f64) / F::cast_from(3.0_f64) * t45688;
    let t45690 = t626 * t12809;
    let t45697 = t92 * t9384;
    (t45656, t45659, t45660, t45662, t45676, t45689, t45690, t45697)
}
