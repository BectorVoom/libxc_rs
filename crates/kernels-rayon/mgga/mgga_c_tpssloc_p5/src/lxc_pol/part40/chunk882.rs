//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 882/1303 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk882(t2180: f64, t671: f64, t1401: f64, t3938: f64, t3941: f64, t577: f64, t8143: f64, t8153: f64, t8161: f64, t1774: f64, t1453: f64, t8129: f64) -> (f64, f64, f64, f64) {
    let t8166 = t2180 * t671;
    let t8171 = 0.45e1_f64 * t8153 * t577 + 0.135e2_f64 * t8161 * t671 + 0.135e2_f64 * t3938 * t2180 + 27.0_f64 * t3941 * t8166 + 0.135e2_f64 * t1401 * t8143;
    let t8221 = t1774 * t2180;
    let t8223 = t8129 * t1453;
    (t8166, t8171, t8221, t8223)
}
