//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 594/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk594(t2379: f64, t4939: f64, t1096: f64, t1113: f64, t1614: f64, t236: f64, t679: f64, t3771: f64, t6: f64, t213: f64, t51: f64, t1109: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4940 = t2379 * t4939;
    let t4943 = t1096 * t1113;
    let t4947 = t236 * t1614;
    let t4948 = t4947 * t679;
    let t4949 = t3771 * t4948;
    let t4950 = t1096 * t6;
    let t4951 = t51 * t213;
    let t4952 = t4951 * t1109;
    (t4940, t4943, t4947, t4949, t4950, t4951, t4952)
}
