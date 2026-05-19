//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1111/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1111<F: Float>(t4372: F, t7647: F, t1427: F, t1983: F, t34186: F, t7586: F, t1545: F, t30948: F, t31026: F, t35204: F, t35208: F, t35211: F, t35213: F, t35215: F, t35219: F, t35223: F, t35228: F, t35231: F, t35232: F, t35234: F, t35238: F, t35240: F, t35242: F) -> (F, F) {
    let t35244 = t7647 * t4372;
    let t35246 = t1983 * t1427;
    let t35248 = t34186 * t7586 * t35246;
    let t35249 = F::cast_from(0.42874018118069736972e-2_f64) * t35248;
    let t35250 = t30948 * t1545;
    let t35251 = F::cast_from(0.16006300097412701803e-1_f64) * t35250;
    let t35253 = -F::cast_from(0.69884649532453671262e-2_f64) * t35204 + F::cast_from(0.15724046144802076034e-2_f64) * t35208 - t35211 + t35213 + F::cast_from(0.21437009059034868486e-2_f64) * t35215 + F::cast_from(0.21437009059034868486e-2_f64) * t35219 + F::cast_from(0.10718504529517434243e-2_f64) * t35223 + t35228 + t35231 + F::cast_from(0.17149607247227894789e-2_f64) * t35232 - F::cast_from(0.42874018118069736972e-3_f64) * t35234 - F::cast_from(0.10718504529517434243e-2_f64) * t35238 - F::cast_from(0.64311027177104605458e-2_f64) * t35240 - F::cast_from(0.25724410870841842183e-2_f64) * t35242 + F::cast_from(0.42874018118069736972e-3_f64) * t35244 + t35249 - t35251 + F::new(0.1528125e-1) * t31026;
    (t35246, t35253)
}
