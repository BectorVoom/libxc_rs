//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 609/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk609<F: Float>(t4972: F, t5203: F, t1800: F, t1869: F, t1693: F, t4827: F, t5057: F, t5066: F, t5071: F, t5075: F, t5078: F, t5080: F, t5178: F, t5189: F, t5197: F, t5201: F) -> (F, F, F, F) {
    let t5204 = t5203 * t4972;
    let t5205 = t1800 * t5204;
    let t5206 = t1869 * t5205;
    let t5210 = F::new(0.27636574074074074073e-2) * t5057 + F::new(0.49745833333333333332e-2) * t5066 - F::new(0.33163888888888888888e-2) * t5071 + F::new(0.22109259259259259258e-2) * t5075 + F::new(0.33163888888888888888e-2) * t5078 + F::new(0.33163888888888888888e-2) * t5080 + F::new(0.24872916666666666666e-2) * t5178 - F::new(0.33163888888888888888e-2) * t5189 + F::new(0.22109259259259259258e-2) * t5197 - F::new(0.33163888888888888888e-2) * t5201 - F::new(0.55273148148148148147e-3) * t5206 + F::new(0.193e0) * t1693 * t4827;
    (t5204, t5205, t5206, t5210)
}
