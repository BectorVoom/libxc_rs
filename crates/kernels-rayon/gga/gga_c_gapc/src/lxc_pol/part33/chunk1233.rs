//! GGA_C_GAPC lxc pol — lxc_pol part 33 (v4rho2sigma2_12) CSE chunk 1233/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part33_v4rho2sigma2_12_chunk1233(t11587: f64, t27940: f64, t2993: f64, t11604: f64, t27868: f64, t33748: f64, t8843: f64, t33152: f64, t9256: f64, t26034: f64, t35050: f64, t33373: f64, t5395: f64, t5974: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t35266 = t2993 * t11587 * t27940;
    let t35269 = t11604 * t27868;
    let t35272 = t2993 * t33748 * t8843;
    let t35275 = t2993 * t33152 * t9256;
    let t35277 = t35050 * t26034;
    let t35280 = t5395 * t33373 * t5974;
    (t35266, t35269, t35272, t35275, t35277, t35280)
}
