//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 375/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk375<F: Float>(t184: F, t2304: F, t21: F, t363: F, t649: F, t1537: F, t947: F, t1546: F, t89: F, t921: F, t1557: F, t920: F) -> (F, F, F, F, F, F) {
    let t2305 = t2304 * t184;
    let t2306 = t2305 * t21;
    let t2309 = t649 * t363;
    let t2976 = t1537 * t947;
    let t2981 = t89 * t1546 * t921;
    let t2983 = t1557 * t920;
    (t2305, t2306, t2309, t2976, t2981, t2983)
}
