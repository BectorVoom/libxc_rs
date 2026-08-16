//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 927/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk927(t2842: f64, t7021: f64, t14763: f64, t7005: f64, t22511: f64, t33939: f64, t4113: f64, t7003: f64, t19100: f64, t4061: f64, t19116: f64, t280: f64, t5009: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t126613 = t2842 * t7021;
    let t127111 = t14763 * t7005;
    let t127359 = t33939 * t22511;
    let t127360 = t4113 * t127359;
    let t127456 = t7003 * t127359;
    let t127560 = t4061 * t19100;
    let t127614 = t19116 * t19100;
    let t127649 = t280 * t5009;
    (t126613, t127111, t127360, t127456, t127560, t127614, t127649)
}
