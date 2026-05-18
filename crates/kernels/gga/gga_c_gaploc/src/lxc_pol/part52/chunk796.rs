//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 796/1013 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk796<F: Float>(t32357: F, t5539: F, t9647: F, t32436: F, t2558: F, t32743: F, t7064: F, t7069: F, t8878: F, t10657: F, t871: F, t33360: F) -> (F, F, F, F, F, F) {
    let t42988 = t9647 * t5539 * t32357;
    let t42991 = t9647 * t5539 * t32436;
    let t43027 = t9647 * t32743 * t2558;
    let t43042 = t7064 * t8878 * t7069;
    let t43072 = t10657 * t871;
    let t43093 = t9647 * t33360 * t2558;
    (t42988, t42991, t43027, t43042, t43072, t43093)
}
