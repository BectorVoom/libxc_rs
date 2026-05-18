//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 776/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk776<F: Float>(t3103: F, t920: F, t1903: F, t1902: F, t18: F, t942: F, t11902: F, t3200: F, t11906: F, t3183: F, t4589: F, t487: F) -> (F, F, F, F, F) {
    let t16060 = t920 * t3103;
    let t16061 = t1903 * t16060;
    let t16062 = t1902 * t16061;
    let t16065 = t18 * t942;
    let t16066 = t1903 * t16065;
    let t16067 = t1902 * t16066;
    let t16070 = t11902 * t3200;
    let t16073 = t11906 * t3183;
    let t16076 = t487 * t4589;
    (t16062, t16067, t16070, t16073, t16076)
}
