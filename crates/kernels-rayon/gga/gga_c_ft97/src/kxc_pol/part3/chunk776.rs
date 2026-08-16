//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 776/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk776(t3103: f64, t920: f64, t1903: f64, t1902: f64, t18: f64, t942: f64, t11902: f64, t3200: f64, t11906: f64, t3183: f64, t4589: f64, t487: f64) -> (f64, f64, f64, f64, f64) {
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
