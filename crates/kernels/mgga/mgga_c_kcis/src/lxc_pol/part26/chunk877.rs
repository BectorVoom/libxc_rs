//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 877/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk877<F: Float>(t12281: F, t20998: F, t4160: F, t2001: F, t833: F, t5440: F, t15878: F, t5426: F, t5661: F, t12286: F, t15844: F, t20970: F, t20977: F, t20982: F, t20987: F, t20991: F, t20996: F, t7043: F) -> (F, F, F, F, F, F) {
    let t20999 = t12281 * t20998;
    let t21000 = t4160 * t20999;
    let t21002 = t2001 * t833;
    let t21003 = t5440 * t21002;
    let t21004 = t15878 * t21003;
    let t21005 = t4160 * t21004;
    let t21007 = t5426 * t21002;
    let t21008 = t15878 * t21007;
    let t21009 = t5661 * t21008;
    let t21011 = -F::new(0.11054629629629629629e-2) * t20970 + F::new(0.890445125e-2) * t12286 * t7043 + F::new(0.33163888888888888888e-2) * t20977 - F::new(0.22109259259259259259e-2) * t20982 + F::new(0.99491666666666666664e-2) * t20987 + F::new(0.13265555555555555555e-1) * t20991 + F::new(0.22109259259259259259e-2) * t15844 + F::new(0.66327777777777777776e-2) * t20996 - F::new(0.22109259259259259259e-2) * t21000 + F::new(0.66327777777777777776e-2) * t21005 - F::new(0.55273148148148148147e-2) * t21009;
    (t21000, t21003, t21005, t21007, t21009, t21011)
}
