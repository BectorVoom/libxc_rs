//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1195/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1195<F: Float>(t28636: F, t28677: F, t1055: F, t1599: F, t7561: F, t25406: F, t7565: F, t1922: F, t5838: F, t1955: F, t5919: F, t10165: F) -> (F, F, F, F, F, F) {
    let t28678 = t28636 + t28677;
    let t28679 = t1055 * t28678;
    let t28681 = t1599 * t7561;
    let t28684 = t25406 * t7565;
    let t28691 = t5838 * t1922;
    let t28696 = t1955 * t5919;
    let t28697 = t10165 * t28696;
    (t28678, t28679, t28681, t28684, t28691, t28697)
}
