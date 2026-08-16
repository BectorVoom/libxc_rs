//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 1285/1445 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk1285(t2993: f64, t33152: f64, t9256: f64, t26034: f64, t35050: f64, t33373: f64, t5395: f64, t5974: f64, t1030: f64, t9262: f64, t11357: f64, t26102: f64) -> (f64, f64, f64, f64, f64) {
    let t35275 = t2993 * t33152 * t9256;
    let t35277 = t35050 * t26034;
    let t35280 = t5395 * t33373 * t5974;
    let t35283 = t1030 * t33152 * t9262;
    let t35285 = t11357 * t26102;
    (t35275, t35277, t35280, t35283, t35285)
}
