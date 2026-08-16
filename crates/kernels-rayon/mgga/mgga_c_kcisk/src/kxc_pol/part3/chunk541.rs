//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 541/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk541(t4340: f64, t4527: f64, t1607: f64, t1610: f64, t1609: f64, t554: f64, t551: f64, t1620: f64, t4176: f64, t4183: f64, t4186: f64, t4190: f64, t4194: f64, t4198: f64, t4201: f64, t4206: f64, t4212: f64, t4216: f64, t4218: f64, t4220: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4528 = t4340 + t4527;
    let t4530 = t1607 * t1610;
    let t4534 = 1.0_f64 / t1609 / t554;
    let t4535 = t551 * t4534;
    let t4536 = t1620 * t1620;
    let t4551 = 0.625e-1_f64 * t4176 - 0.34173611111111111111e0_f64 * t4183 + 0.14388888888888888889e0_f64 * t4186 + 0.101171875e-1_f64 * t4190 - 0.13489583333333333333e-1_f64 * t4194 - 0.9375e-1_f64 * t4198 + 0.5e0_f64 * t4201 - 0.125e0_f64 * t4206 + 0.1875e0_f64 * t4212 - 0.1875e0_f64 * t4216 + 0.10791666666666666667e0_f64 * t4218 - 0.26979166666666666666e-1_f64 * t4220;
    (t4528, t4530, t4534, t4535, t4536, t4551)
}
