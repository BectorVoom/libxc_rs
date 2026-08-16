//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1764/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1764<F: Float>(t63: F, t9365: F, t2752: F, t606: F, t22641: F, t9523: F, t22690: F, t6639: F, t1887: F, t23069: F) -> (F, F, F, F, F) {
    let t81446 = t63 * t9365;
    let t81547 = t2752 * t606;
    let t81573 = t22641 * t9523;
    let t81575 = t81573 * t22690 * t6639;
    let t81591 = t23069 * t1887;
    (t81446, t81547, t81573, t81575, t81591)
}
