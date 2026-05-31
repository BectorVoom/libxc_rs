//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 464/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk464<F: Float>(t1433: F, t3619: F, t457: F, t1216: F, t3571: F, t3573: F, t3577: F, t3581: F, t3585: F, t321: F, t1167: F, t1171: F) -> (F, F, F, F, F, F) {
    let t3620 = t1433 * t3619;
    let t3621 = t457 * t3620;
    let t3624 = t1216 * t1216;
    let t3626 = F::cast_from(0.23744444444444444444e-1_f64) * t3571;
    let t3631 = t3626 + F::cast_from(0.11872222222222222222e-1_f64) * t3573 - F::cast_from(0.11872222222222222222e-1_f64) * t3577 + F::cast_from(0.35616666666666666666e-1_f64) * t3581 - F::cast_from(0.17808333333333333333e-1_f64) * t3585;
    let t3633 = F::cast_from(0.62182e-1_f64) * t3631 * t321;
    let t3634 = t1167 * t1171;
    (t3620, t3621, t3624, t3631, t3633, t3634)
}
