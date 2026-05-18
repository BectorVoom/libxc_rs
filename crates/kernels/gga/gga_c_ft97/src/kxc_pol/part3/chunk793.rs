//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 793/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk793<F: Float>(t3271: F, t925: F, t8557: F, t11468: F, t15951: F, t363: F, t979: F, t2983: F, t11556: F, t11552: F, t15955: F, t3214: F) -> (F, F, F, F, F, F) {
    let t16305 = t925 * t3271;
    let t16306 = t8557 * t16305;
    let t16309 = t11468 * t15951;
    let t16312 = t979 * t363;
    let t16313 = t2983 * t16312;
    let t16314 = t11556 * t16313;
    let t16317 = t11552 * t15955;
    let t16320 = t925 * t3214;
    (t16306, t16309, t16312, t16314, t16317, t16320)
}
