//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1256/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1256<F: Float>(t322: F, t44630: F, t44641: F, t12829: F, t833: F, t1013: F, t1120: F, t11220: F, t12244: F, t1300: F, t2394: F, t2941: F, t2944: F, t327: F, t3506: F, t3509: F, t38839: F, t41901: F, t829: F, t834: F, t9676: F) -> (F, F) {
    let t324 = F::cast_from(0.0_f64) < t322;
    let t44642 = t44630 + t44641;
    let t44643 = piecewise3::<F>(t324, F::cast_from(0.0_f64), t44642);
    let t44646 = t12829 * t833;
    let t44661 = -F::cast_from(0.128e1_f64) * t1300 * t3506 * t2941 - F::cast_from(0.128e1_f64) * t1300 * t1120 * t9676 - F::cast_from(0.128e1_f64) * t1300 * t12829 * t829 - F::cast_from(0.64e0_f64) * t44643 * t327 - F::cast_from(0.128e1_f64) * t44646 * t829 - F::cast_from(0.256e1_f64) * t41901 * t1013 - F::cast_from(0.256e1_f64) * t12244 * t2394 - F::cast_from(0.384e1_f64) * t38839 * t2944 - F::cast_from(0.128e1_f64) * t11220 * t2941 - F::cast_from(0.128e1_f64) * t3509 * t9676 - F::cast_from(0.64e0_f64) * t834 * t44643;
    (t44642, t44661)
}
