//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 509/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk509<F: Float>(t1528: F, t920: F, t72: F, t942: F, t1524: F, t1526: F, t1527: F, t342: F, t343: F, t948: F, t947: F) -> (F, F, F, F, F) {
    let t4406 = t1528 * t920;
    let t4410 = t72 * t942;
    let t4414 = t948 - t1524 - t1526 * t1527 * t4406 / 12.0 - t342 * t343 * t4410 / 4.0;
    let t4415 = t4414 * t947;
    let t4417 = t920 * t920;
    (t4406, t4410, t4414, t4415, t4417)
}
