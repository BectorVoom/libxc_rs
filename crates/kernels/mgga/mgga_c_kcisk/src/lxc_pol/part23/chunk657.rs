//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 657/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk657<F: Float>(t1354: F, t5703: F, t3573: F, t3626: F, t5668: F, t5673: F, t5678: F, t5682: F, t321: F, t1171: F, t2079: F, t1192: F, t2093: F, t3634: F, t1190: F, t3639: F) -> (F, F, F, F, F, F, F, F) {
    let t5704 = t1354 * t5703;
    let t5712 = t3626 + 0.5936111111111111111e-2 * t3573 + 0.5936111111111111111e-2 * t5668 - 0.11872222222222222222e-1 * t5673 + 0.35616666666666666666e-1 * t5678 - 0.35616666666666666666e-1 * t5682;
    let t5714 = 0.62182e-1 * t5712 * t321;
    let t5715 = t2079 * t1171;
    let t5717 = 1.0 * t5715 * t1192;
    let t5719 = 1.0 * t3634 * t2093;
    let t5720 = t2093 * t1190;
    let t5722 = 2.0 * t3639 * t5720;
    (t5704, t5712, t5714, t5715, t5717, t5719, t5720, t5722)
}
