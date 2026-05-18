//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 824/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk824<F: Float>(t16812: F, t16824: F, t550: F, t133: F, t3347: F, t4699: F, t4441: F, t8690: F, t120: F, t3056: F, t15647: F, t378: F) -> (F, F, F, F, F) {
    let t16825 = t16812 + t16824;
    let t16826 = t550 * t16825;
    let t16827 = t133 * t16826;
    let t16830 = t3347 * t4699;
    let t16832 = t8690 * t4441;
    let t16835 = t120 * t3056;
    let t16839 = t378 * t15647 * t120;
    (t16827, t16830, t16832, t16835, t16839)
}
