//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 660/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk660<F: Float>(t776: F, t780: F, t9183: F, t2629: F, t2633: F, t41: F, t8616: F, t2442: F, t2620: F, t525: F, t642: F, t773: F, t8781: F, t8787: F) -> (F, F, F, F) {
    let t777 = t776 < -F::new(0.66725e-1);
    let t9184 = t780 * t9183;
    let t9189 = t2629 * t2633;
    let t9192 = t8616 * t41;
    let t9206 = piecewise3::<F>(t777, F::new(0.0), F::new(10.0) / F::new(9.0) * t525 * t9192 * t642 - F::new(20.0) / F::new(27.0) * t525 * t2620 * t2442 + F::new(40.0) / F::new(81.0) * t525 * t773 * t8781 - F::new(10.0) / F::new(27.0) * t525 * t773 * t8787);
    (t9184, t9189, t9192, t9206)
}
