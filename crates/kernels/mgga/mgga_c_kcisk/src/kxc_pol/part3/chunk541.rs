//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 541/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk541<F: Float>(t4340: F, t4527: F, t1607: F, t1610: F, t1609: F, t554: F, t551: F, t1620: F, t4176: F, t4183: F, t4186: F, t4190: F, t4194: F, t4198: F, t4201: F, t4206: F, t4212: F, t4216: F, t4218: F, t4220: F) -> (F, F, F, F, F, F) {
    let t4528 = t4340 + t4527;
    let t4530 = t1607 * t1610;
    let t4534 = F::new(1.0) / t1609 / t554;
    let t4535 = t551 * t4534;
    let t4536 = t1620 * t1620;
    let t4551 = F::new(0.625e-1) * t4176 - F::cast_from(0.34173611111111111111e0_f64) * t4183 + F::cast_from(0.14388888888888888889e0_f64) * t4186 + F::cast_from(0.101171875e-1_f64) * t4190 - F::cast_from(0.13489583333333333333e-1_f64) * t4194 - F::new(0.9375e-1) * t4198 + F::new(0.5e0) * t4201 - F::new(0.125e0) * t4206 + F::new(0.1875e0) * t4212 - F::new(0.1875e0) * t4216 + F::cast_from(0.10791666666666666667e0_f64) * t4218 - F::cast_from(0.26979166666666666666e-1_f64) * t4220;
    (t4528, t4530, t4534, t4535, t4536, t4551)
}
