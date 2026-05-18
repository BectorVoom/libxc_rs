//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 580/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk580<F: Float>(t167: F, t4462: F, t569: F, t2205: F, t4454: F, t1039: F, t2086: F, t91: F, t2097: F, t4511: F, t2102: F, t4656: F) -> (F, F, F, F, F, F) {
    let t4743 = t569 * t167 * t4462;
    let t4747 = t2205 * t167 * t4454;
    let t4753 = t1039 * t1039;
    let t4755 = t91 * t2086 * t4753;
    let t4759 = t2097 * t4511;
    let t4762 = t2102 * t4656;
    (t4743, t4747, t4753, t4755, t4759, t4762)
}
