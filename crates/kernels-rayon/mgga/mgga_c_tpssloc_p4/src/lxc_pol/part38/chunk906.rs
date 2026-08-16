//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 906/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk906(t3: f64, t8240: f64, t1458: f64, t2180: f64, t1401: f64, t3941: f64, t5371: f64, t577: f64, t8161: f64, t8230: f64, t590: f64, t60: f64) -> (f64, f64, f64, f64) {
    let t8241 = t3 * t8240;
    let t8251 = t2180 * t1458;
    let t8256 = 0.45e1_f64 * t8240 * t577 + 0.135e2_f64 * t8161 * t1458 + 0.135e2_f64 * t5371 * t2180 + 27.0_f64 * t3941 * t8251 + 0.135e2_f64 * t1401 * t8230;
    let t8705 = 1.0_f64 / t60 / t590;
    (t8241, t8251, t8256, t8705)
}
