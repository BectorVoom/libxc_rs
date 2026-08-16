//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 769/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk769(t15944: f64, t446: f64, t15737: f64, t8577: f64, t363: f64, t942: f64, t2992: f64, t1564: f64, t2983: f64, t7793: f64, t3266: f64, t925: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t15945 = t446 * t15944;
    let t15947 = t8577 * t15737;
    let t15948 = t446 * t15947;
    let t15950 = t942 * t363;
    let t15951 = t2992 * t15950;
    let t15952 = t1564 * t15951;
    let t15953 = t446 * t15952;
    let t15955 = t2983 * t15950;
    let t15956 = t7793 * t15955;
    let t15957 = t446 * t15956;
    let t15959 = t925 * t3266;
    (t15945, t15948, t15951, t15953, t15955, t15957, t15959)
}
