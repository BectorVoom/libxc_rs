//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 948/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk948<F: Float>(t29730: F, t29753: F, t2647: F, t9162: F, t11966: F, t15951: F, t15953: F, t15955: F, t18779: F, t1994: F, t22265: F, t22328: F, t28211: F, t28219: F, t28222: F, t28226: F, t795: F, t9163: F) -> (F, F, F) {
    let t29754 = t29730 + t29753;
    let t29758 = t9162 * t2647;
    let t29759 = t29758 * t11966;
    let t29770 = F::cast_from(0.34048259259259259259e-1_f64) * t22265 + t29754 * t795 + F::cast_from(0.223494e0_f64) * t18779 * t9163 - F::cast_from(0.386e0_f64) * t1994 * t29759 + F::cast_from(0.69644166666666666666e-2_f64) * t28211 + F::cast_from(0.11607361111111111111e-2_f64) * t15951 - F::cast_from(0.77382407407407407405e-3_f64) * t15953 + F::cast_from(0.30952962962962962963e-2_f64) * t15955 + F::cast_from(0.30952962962962962963e-2_f64) * t22328 - F::cast_from(0.77382407407407407405e-3_f64) * t28219 - F::cast_from(0.12381185185185185185e-1_f64) * t28222 + F::cast_from(0.23214722222222222222e-2_f64) * t28226;
    (t29754, t29759, t29770)
}
