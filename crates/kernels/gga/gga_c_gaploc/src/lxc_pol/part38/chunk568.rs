//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 568/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk568<F: Float>(t4130: F, t986: F, t2482: F, t9272: F, t10231: F, t549: F, t544: F, t8410: F) -> (F, F, F, F, F) {
    let t10608 = t4130 * t986;
    let t10609 = t10608 * t2482;
    let t10610 = t9272 * t10609;
    let t10611 = F::new(0.57514388930881124514e0) * t10610;
    let t10612 = t549 * t10231;
    let t10615 = t544 * t8410;
    (t10608, t10610, t10611, t10612, t10615)
}
