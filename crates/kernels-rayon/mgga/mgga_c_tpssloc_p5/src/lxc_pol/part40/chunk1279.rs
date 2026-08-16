//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 1279/1303 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk1279(t30192: f64, t30215: f64, t3: f64, t112: f64, t8240: f64, t1458: f64, t8143: f64, t2180: f64, t4072: f64, t671: f64, t8230: f64, t12524: f64, t1401: f64, t16521: f64, t16524: f64, t20173: f64, t29993: f64, t29996: f64, t30180: f64, t3938: f64, t3941: f64, t5371: f64, t5376: f64, t577: f64, t8161: f64, t8166: f64, t8251: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t30217 = 2.0_f64 * t30192 + 2.0_f64 * t30215;
    let t30218 = t3 * t30217;
    let t30231 = t8240 * t112;
    let t30250 = t8143 * t1458;
    let t30253 = t2180 * t4072;
    let t30258 = t8230 * t671;
    let t30263 = 0.45e1_f64 * t30217 * t577 + 0.135e2_f64 * t30231 * t671 + 0.135e2_f64 * t29993 * t1458 + 27.0_f64 * t29996 * t5376 + 0.135e2_f64 * t8161 * t4072 + 0.135e2_f64 * t16521 * t2180 + 27.0_f64 * t16524 * t8166 + 0.135e2_f64 * t5371 * t8143 + 27.0_f64 * t12524 * t8251 + 27.0_f64 * t20173 * t8251 + 27.0_f64 * t3941 * t30250 + 27.0_f64 * t3941 * t30253 + 0.135e2_f64 * t3938 * t8230 + 27.0_f64 * t3941 * t30258 + 0.135e2_f64 * t1401 * t30180;
    (t30217, t30218, t30231, t30250, t30253, t30258, t30263)
}
