//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 609/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk609(t4972: f64, t5203: f64, t1800: f64, t1869: f64, t1693: f64, t4827: f64, t5057: f64, t5066: f64, t5071: f64, t5075: f64, t5078: f64, t5080: f64, t5178: f64, t5189: f64, t5197: f64, t5201: f64) -> (f64, f64, f64, f64) {
    let t5204 = t5203 * t4972;
    let t5205 = t1800 * t5204;
    let t5206 = t1869 * t5205;
    let t5210 = 0.27636574074074074073e-2_f64 * t5057 + 0.49745833333333333332e-2_f64 * t5066 - 0.33163888888888888888e-2_f64 * t5071 + 0.22109259259259259258e-2_f64 * t5075 + 0.33163888888888888888e-2_f64 * t5078 + 0.33163888888888888888e-2_f64 * t5080 + 0.24872916666666666666e-2_f64 * t5178 - 0.33163888888888888888e-2_f64 * t5189 + 0.22109259259259259258e-2_f64 * t5197 - 0.33163888888888888888e-2_f64 * t5201 - 0.55273148148148148147e-3_f64 * t5206 + 0.193e0_f64 * t1693 * t4827;
    (t5204, t5205, t5206, t5210)
}
