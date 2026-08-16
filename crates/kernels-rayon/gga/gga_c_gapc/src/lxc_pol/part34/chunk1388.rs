//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 1388/1427 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk1388(t34054: f64, t34056: f64, t34060: f64, t34062: f64, t34036: f64, t36800: f64, t36801: f64, t36802: f64, t36803: f64, t36804: f64, t36805: f64, t34066: f64) -> (f64, f64) {
    let t36806 = 0.28605695478281987903e-5_f64 * t34054;
    let t36807 = 0.14068374825384584215e-7_f64 * t34056;
    let t36808 = 0.46573198186092908864e-9_f64 * t34060;
    let t36809 = 0.49520679385353736436e-5_f64 * t34062;
    let t36810 = -0.11666621455439814816e-3_f64 * t34036 + t36800 - t36801 - t36802 + t36803 - t36804 + t36805 + t36806 + t36807 + t36808 + t36809;
    let t36812 = 0.67528199161846004232e-6_f64 * t34066;
    (t36810, t36812)
}
