//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 821/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk821<F: Float>(t15911: F, t4162: F, t15909: F, t3734: F, t5628: F, t1464: F, t1650: F, t4124: F, t4163: F, t12241: F, t4160: F, t11832: F, t11838: F, t15851: F, t15854: F, t15858: F, t15863: F, t15868: F, t15872: F, t15876: F, t15881: F, t15885: F, t15890: F, t15894: F, t15896: F, t15901: F, t15905: F) -> (F, F, F, F, F) {
    let t15912 = t4162 * t15911;
    let t15913 = t15909 * t15912;
    let t15915 = t3734 * t5628;
    let t15916 = t1464 * t15915;
    let t15919 = t4163 * t1650 * t4124;
    let t15920 = t12241 * t15919;
    let t15921 = t4160 * t15920;
    let t15923 = F::cast_from(0.99491666666666666664e-2_f64) * t15851 - F::cast_from(0.33163888888888888888e-2_f64) * t15854 + F::cast_from(0.88437037037037037034e-2_f64) * t15858 + F::cast_from(0.55273148148148148147e-3_f64) * t15863 - F::cast_from(0.44218518518518518517e-2_f64) * t15868 + F::cast_from(0.3684876543209876543e-2_f64) * t15872 - F::cast_from(0.22109259259259259258e-2_f64) * t15876 + F::cast_from(0.66327777777777777776e-2_f64) * t15881 - F::cast_from(0.55273148148148148146e-2_f64) * t15885 + F::cast_from(0.66327777777777777776e-2_f64) * t15890 - F::cast_from(0.16581944444444444444e-2_f64) * t11832 + F::cast_from(0.33163888888888888888e-2_f64) * t15894 - F::cast_from(0.5895802469135802469e-2_f64) * t15896 - F::cast_from(0.16581944444444444444e-2_f64) * t15901 - F::cast_from(0.44218518518518518517e-2_f64) * t15905 - F::cast_from(0.22109259259259259258e-2_f64) * t11838 + F::cast_from(0.11054629629629629629e-2_f64) * t15913 - F::cast_from(0.49745833333333333332e-2_f64) * t15916 + F::cast_from(0.33163888888888888888e-2_f64) * t15921;
    (t15913, t15916, t15919, t15921, t15923)
}
