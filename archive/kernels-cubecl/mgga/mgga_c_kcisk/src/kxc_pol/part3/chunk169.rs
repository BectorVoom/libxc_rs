//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 169/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk169<F: Float>(t657: F, t9: F, t604: F) -> (F, F, F) {
    let t658 = t9 * t657;
    let t659 = F::cast_from(0.0_f64) < t604;
    let t661 = piecewise3::<F>(t659, t604, -t604);
    let t662 = F::cast_from(1.0_f64) / t661;
    (t658, t661, t662)
}
