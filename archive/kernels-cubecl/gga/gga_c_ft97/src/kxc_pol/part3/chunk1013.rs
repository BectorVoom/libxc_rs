//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 1013/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk1013<F: Float>(t1775: F, t5352: F, t5343: F, t2: F, t5299: F, t2681: F, t824: F, t4129: F, t4218: F, t18307: F, t848: F, t5225: F) -> (F, F, F, F, F, F) {
    let t19693 = t1775 * t5352;
    let t19695 = t1775 * t5343;
    let t19697 = t2 * t5299;
    let t19699 = t2681 * t19697 * t824;
    let t19703 = t2681 * t4218 * t4129;
    let t19706 = t848 * t18307;
    let t19709 = t2 * t5225;
    (t19693, t19695, t19699, t19703, t19706, t19709)
}
