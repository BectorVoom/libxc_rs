//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 827/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk827<F: Float>(t16771: F, t16774: F, t1497: F, t167: F, t4171: F, t4170: F, t833: F) -> (F, F, F, F) {
    let t16775 = t16771 * t16774;
    let t16777 = t167 * t1497;
    let t16778 = t4171 * t16777;
    let t16779 = t4170 * t16778;
    let t16780 = t16771 * t16779;
    let t16782 = t167 * t833;
    (t16775, t16778, t16780, t16782)
}
