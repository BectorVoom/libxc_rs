//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 962/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk962<F: Float>(t17054: F, t2507: F, t5060: F, t5064: F, t1869: F, t4811: F, t6978: F, t1871: F, t6943: F, t1895: F, t7071: F, t11233: F, t11241: F, t17016: F, t17018: F, t17021: F, t17022: F, t17025: F, t17029: F, t17034: F, t17038: F, t17042: F, t17047: F, t17051: F, t5044: F, t7278: F, sigma2: F) -> (F, F, F, F, F, F, F, F, F) {
    let t17055 = 0.33163888888888888888e-2 * t17054;
    let t17056 = t2507 * t5060;
    let t17057 = t17056 * sigma2;
    let t17058 = t17057 * t5064;
    let t17059 = t1869 * t17058;
    let t17061 = t4811 * t6978;
    let t17064 = t6943 * t1871;
    let t17065 = t17064 * sigma2;
    let t17066 = t17065 * t1895;
    let t17067 = t1869 * t17066;
    let t17069 = t4811 * t7071;
    let t17070 = 0.33163888888888888888e-2 * t17069;
    let t17071 = -0.193e0 * t7278 * t5044 - 0.22109259259259259258e-2 * t17016 - 0.33163888888888888888e-2 * t17018 + t17021 - 0.5895802469135802469e-2 * t17022 + 0.33163888888888888888e-2 * t17025 + 0.3684876543209876543e-2 * t17029 + 0.66327777777777777776e-2 * t17034 - 0.22109259259259259258e-2 * t17038 - 0.22109259259259259258e-2 * t17042 + 0.66327777777777777776e-2 * t17047 - 0.55273148148148148146e-2 * t17051 - 0.3684876543209876543e-3 * t11233 - t17055 + 0.49745833333333333332e-2 * t17059 + 0.22109259259259259258e-2 * t17061 + 0.33163888888888888888e-2 * t11241 - 0.49745833333333333332e-2 * t17067 - t17070;
    (t17056, t17057, t17059, t17061, t17064, t17065, t17067, t17069, t17071)
}
