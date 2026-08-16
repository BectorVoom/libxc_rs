//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1325/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1325(t19109: f64, t919: f64, t10063: f64, t10108: f64, t10112: f64, t10115: f64, t10117: f64, t10121: f64, t10226: f64, t10340: f64, t1167: f64, t22979: f64, t23075: f64, t23081: f64, t2380: f64, t2381: f64, t27001: f64, t28121: f64, t28123: f64, t2886: f64, t2888: f64, t3026: f64, t3174: f64, t3177: f64, t31857: f64, t3207: f64, t3324: f64, t3898: f64, t406: f64, t8319: f64) -> (f64, f64) {
    let t32078 = t19109 * t919;
    let t32097 = 11.0_f64 / 18.0_f64 * t3324 * t2886 * t3177 - t10063 * t10117 / 6.0_f64 + t3174 * t2888 * t27001 * t1167 / 16.0_f64 + t3174 * t2888 * t10115 * t3026 / 16.0_f64 + t10063 * t10108 / 2.0_f64 - t10063 * t10112 / 3.0_f64 + 0.28582678745379824648e-3_f64 * t28121 - 0.30488190661738479624e-2_f64 * t28123 + 0.51448821741683684368e-2_f64 * t23075 * t406 * t31857 * t32078 - 0.77173232612525526552e-2_f64 * t23081 * t406 * t31857 * t10121 - 0.20579528696673473747e-1_f64 * t8319 * t10226 - 0.21437009059034868486e-3_f64 * t22979 * t406 * t31857 * t3207 - 0.12862205435420921092e-2_f64 * t2380 * t2381 * t10340 * t3898;
    (t32078, t32097)
}
