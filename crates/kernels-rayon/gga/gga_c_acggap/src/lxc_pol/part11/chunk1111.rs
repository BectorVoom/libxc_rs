//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1111/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1111(t4372: f64, t7647: f64, t1427: f64, t1983: f64, t34186: f64, t7586: f64, t1545: f64, t30948: f64, t31026: f64, t35204: f64, t35208: f64, t35211: f64, t35213: f64, t35215: f64, t35219: f64, t35223: f64, t35228: f64, t35231: f64, t35232: f64, t35234: f64, t35238: f64, t35240: f64, t35242: f64) -> (f64, f64) {
    let t35244 = t7647 * t4372;
    let t35246 = t1983 * t1427;
    let t35248 = t34186 * t7586 * t35246;
    let t35249 = 0.42874018118069736972e-2_f64 * t35248;
    let t35250 = t30948 * t1545;
    let t35251 = 0.16006300097412701803e-1_f64 * t35250;
    let t35253 = -0.69884649532453671262e-2_f64 * t35204 + 0.15724046144802076034e-2_f64 * t35208 - t35211 + t35213 + 0.21437009059034868486e-2_f64 * t35215 + 0.21437009059034868486e-2_f64 * t35219 + 0.10718504529517434243e-2_f64 * t35223 + t35228 + t35231 + 0.17149607247227894789e-2_f64 * t35232 - 0.42874018118069736972e-3_f64 * t35234 - 0.10718504529517434243e-2_f64 * t35238 - 0.64311027177104605458e-2_f64 * t35240 - 0.25724410870841842183e-2_f64 * t35242 + 0.42874018118069736972e-3_f64 * t35244 + t35249 - t35251 + 0.1528125e-1_f64 * t31026;
    (t35246, t35253)
}
