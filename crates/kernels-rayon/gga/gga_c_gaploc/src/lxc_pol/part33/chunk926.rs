//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 926/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk926(t10144: f64, t2343: f64, t2268: f64, t3327: f64, t6313: f64, t2317: f64, t2761: f64, t6525: f64, t2321: f64, t8237: f64, t9074: f64, t123: f64, t7887: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10145 = t2343 * t10144;
    let t10147 = 0.56910013271352299198e-1_f64 * t2268 * t10145;
    let t10150 = 0.37940008847568199465e-1_f64 * t6313 * t3327;
    let t10160 = t2761 * t2317;
    let t10161 = t6525 * t10160;
    let t10162 = 0.11856252764865062333e-2_f64 * t10161;
    let t10163 = t8237 * t2321;
    let t10164 = t9074 * t10163;
    let t10165 = 0.11856252764865062333e-2_f64 * t10164;
    let t10166 = t7887 * t123;
    (t10145, t10147, t10150, t10160, t10162, t10163, t10165, t10166)
}
