//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 463/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk463<F: Float>(t3598: F, t420: F, t1173: F, t1361: F, t3559: F, t3587: F, t3571: F, t3573: F, t3577: F, t3581: F, t3585: F, t1175: F, t1355: F, t306: F) -> (F, F, F, F, F, F) {
    let t3599 = t3598 * t420;
    let t3602 = t1173 * t1361;
    let t3607 = t3598 * t3559;
    let t3609 = t1173 * t3587;
    let t3611 = F::cast_from(0.55033333333333333333e-2_f64) * t3571;
    let t3616 = -F::new(0.991e-2) * t3607 + F::new(0.1982e-1) * t3609 + t3611 + F::cast_from(0.27516666666666666666e-2_f64) * t3573 - F::cast_from(0.27516666666666666667e-2_f64) * t3577 + F::new(0.8255e-2) * t3581 - F::new(0.41275e-2) * t3585;
    let t3619 = -t3599 * t3559 / F::new(8.0) + t3602 * t1175 / F::new(2.0) + t1355 * t3587 / F::new(4.0) + t306 * t3616 / F::new(2.0);
    (t3599, t3602, t3607, t3609, t3616, t3619)
}
