//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 577/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk577<F: Float>(t4714: F, t526: F, t27: F, t89: F, t1957: F, t3530: F, t3535: F, t4654: F, t4658: F, t4662: F, t4666: F, t4671: F) -> (F, F, F) {
    let t4715 = t526 * t4714;
    let t4717 = t89 * t27 * t4715;
    let t4719 = t1957 + t3530 + t3535 - t4654 / F::new(27.0) + t4658 / F::new(9.0) + t4662 / F::new(9.0) - t4666 / F::new(18.0) + t4671 / F::new(3.0) - t4717 / F::new(6.0);
    (t4715, t4717, t4719)
}
