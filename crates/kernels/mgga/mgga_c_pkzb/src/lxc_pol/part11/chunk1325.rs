//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1325/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1325<F: Float>(t19109: F, t919: F, t10063: F, t10108: F, t10112: F, t10115: F, t10117: F, t10121: F, t10226: F, t10340: F, t1167: F, t22979: F, t23075: F, t23081: F, t2380: F, t2381: F, t27001: F, t28121: F, t28123: F, t2886: F, t2888: F, t3026: F, t3174: F, t3177: F, t31857: F, t3207: F, t3324: F, t3898: F, t406: F, t8319: F) -> (F, F) {
    let t32078 = t19109 * t919;
    let t32097 = F::cast_from(11.0_f64) / F::cast_from(18.0_f64) * t3324 * t2886 * t3177 - t10063 * t10117 / F::cast_from(6.0_f64) + t3174 * t2888 * t27001 * t1167 / F::cast_from(16.0_f64) + t3174 * t2888 * t10115 * t3026 / F::cast_from(16.0_f64) + t10063 * t10108 / F::cast_from(2.0_f64) - t10063 * t10112 / F::cast_from(3.0_f64) + F::cast_from(0.28582678745379824648e-3_f64) * t28121 - F::cast_from(0.30488190661738479624e-2_f64) * t28123 + F::cast_from(0.51448821741683684368e-2_f64) * t23075 * t406 * t31857 * t32078 - F::cast_from(0.77173232612525526552e-2_f64) * t23081 * t406 * t31857 * t10121 - F::cast_from(0.20579528696673473747e-1_f64) * t8319 * t10226 - F::cast_from(0.21437009059034868486e-3_f64) * t22979 * t406 * t31857 * t3207 - F::cast_from(0.12862205435420921092e-2_f64) * t2380 * t2381 * t10340 * t3898;
    (t32078, t32097)
}
