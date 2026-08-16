//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1772/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1772<F: Float>(t23124: F, t81902: F, t23138: F, t6604: F, t6606: F, t22690: F, t2627: F, t10024: F, t1899: F, t2693: F, t6609: F, t213: F, t6589: F, t9223: F) -> (F, F, F, F, F, F, F) {
    let t81903 = t81902 * t23124;
    let t81911 = t23138 * t6604;
    let t81912 = t81911 * t6606;
    let t81914 = t22690 * t2627;
    let t81920 = t1899 * t10024;
    let t81928 = t6609 * t2693;
    let t81933 = t9223 * t6589 * t213;
    (t81903, t81911, t81912, t81914, t81920, t81928, t81933)
}
