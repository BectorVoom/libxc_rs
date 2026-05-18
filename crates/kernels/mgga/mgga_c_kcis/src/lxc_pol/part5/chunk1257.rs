//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1257/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1257<F: Float>(t15878: F, t21003: F, t4160: F, t21002: F, t5426: F, t5661: F, t12286: F, t15844: F, t20970: F, t20977: F, t20982: F, t20987: F, t20991: F, t20996: F, t21000: F, t7043: F) -> (F, F, F) {
    let t21004 = t15878 * t21003;
    let t21005 = t4160 * t21004;
    let t21007 = t5426 * t21002;
    let t21008 = t15878 * t21007;
    let t21009 = t5661 * t21008;
    let t21011 = -F::new(0.11054629629629629629e-2) * t20970 + F::new(0.890445125e-2) * t12286 * t7043 + F::new(0.33163888888888888888e-2) * t20977 - F::new(0.22109259259259259259e-2) * t20982 + F::new(0.99491666666666666664e-2) * t20987 + F::new(0.13265555555555555555e-1) * t20991 + F::new(0.22109259259259259259e-2) * t15844 + F::new(0.66327777777777777776e-2) * t20996 - F::new(0.22109259259259259259e-2) * t21000 + F::new(0.66327777777777777776e-2) * t21005 - F::new(0.55273148148148148147e-2) * t21009;
    (t21005, t21009, t21011)
}
