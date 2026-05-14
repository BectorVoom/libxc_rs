//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 996/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk996<F: Float>(t148408: F, t446: F, t9073: F, t148336: F, t1969: F, t139497: F, t3188: F, t9049: F, t1369: F, t147944: F, t2112: F, t28: F, t1039: F, t32869: F, t586: F, t5890: F) -> (F, F, F, F, F, F) {
    let t148470 = t446 * t9073 * t148408;
    let t148473 = t446 * t1969 * t148336;
    let t148475 = t139497 * t3188;
    let t148477 = t446 * t9049 * t148475;
    let t148481 = t1369 * t28 * t2112 * t147944;
    let t148486 = t5890 * t28 * t586 * t32869 * t1039;
    (t148470, t148473, t148475, t148477, t148481, t148486)
}
