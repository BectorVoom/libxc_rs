//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 580/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk580<F: Float>(t1394: F, t5773: F, t1947: F, t3738: F, t1392: F, t4992: F, t86: F) -> (F, F, F, F) {
    let t5774 = t1394 * t5773;
    let t5776 = t3738 * t1947;
    let t5777 = t1394 * t5776;
    let t5780 = t86 * t4992 * t1392;
    (t5774, t5776, t5777, t5780)
}
