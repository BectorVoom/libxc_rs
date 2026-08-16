//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2220/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2220<F: Float>(t1933: F, t23479: F, t88405: F, t1409: F, t1937: F, t6722: F, t1015: F, t10475: F, t13762: F, t14041: F, t1615: F, t23419: F, t23678: F, t25652: F, t25653: F, t25658: F, t25660: F, t25661: F, t3040: F, t3120: F, t360: F, t4575: F, t4579: F, t4649: F, t82516: F, t82542: F, t82754: F, t83008: F, t83134: F, t88537: F, t88655: F) -> F {
    let t88689 = F::cast_from(0.20186378047070195428e-3_f64) * t1933 * t88405 * t23479;
    let t88692 = F::cast_from(0.16149102437656156342e-2_f64) * t6722 * t1409 * t1937;
    let t88702 = F::cast_from(0.20186378047070195428e-3_f64) * t25652 * t25653 * t23678 * t3120 + F::cast_from(0.60559134141210586284e-3_f64) * t88537 * t10475 * t1615 * t82516 * t3040 - F::cast_from(0.60559134141210586284e-3_f64) * t88537 * t25653 * t82542 * t3040 - F::cast_from(0.20186378047070195428e-3_f64) * t88655 * t25661 - F::cast_from(0.20186378047070195428e-3_f64) * t25652 * t1015 * t4649 * t25660 - F::cast_from(0.10093189023535097714e-3_f64) * t25652 * t25658 * t82754 * t360 - t88689 - t88692 + t83008 * t4579 / F::cast_from(1152.0_f64) + t23419 * t13762 / F::cast_from(1152.0_f64) + t23419 * t14041 / F::cast_from(2304.0_f64) + F::cast_from(0.16149102437656156342e-2_f64) * t83134 + t83008 * t4575 / F::cast_from(1152.0_f64);
    t88702
}
