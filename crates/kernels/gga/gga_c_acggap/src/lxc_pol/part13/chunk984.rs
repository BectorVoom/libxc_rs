//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 984/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk984<F: Float>(t30861: F, t8649: F, t4316: F, t7822: F, t4372: F, t7647: F, t1427: F, t1983: F, t34186: F, t7586: F, t1545: F, t30948: F, t31026: F, t35204: F, t35208: F, t35211: F, t35213: F, t35215: F, t35219: F, t35223: F, t35228: F, t35231: F, t35232: F, t35234: F, t35238: F) -> (F, F) {
    let t35240 = t30861 * t8649;
    let t35242 = t7822 * t4316;
    let t35244 = t7647 * t4372;
    let t35246 = t1983 * t1427;
    let t35248 = t34186 * t7586 * t35246;
    let t35249 = 0.42874018118069736972e-2 * t35248;
    let t35250 = t30948 * t1545;
    let t35251 = 0.16006300097412701803e-1 * t35250;
    let t35253 = -0.69884649532453671262e-2 * t35204 + 0.15724046144802076034e-2 * t35208 - t35211 + t35213 + 0.21437009059034868486e-2 * t35215 + 0.21437009059034868486e-2 * t35219 + 0.10718504529517434243e-2 * t35223 + t35228 + t35231 + 0.17149607247227894789e-2 * t35232 - 0.42874018118069736972e-3 * t35234 - 0.10718504529517434243e-2 * t35238 - 0.64311027177104605458e-2 * t35240 - 0.25724410870841842183e-2 * t35242 + 0.42874018118069736972e-3 * t35244 + t35249 - t35251 + 0.1528125e-1 * t31026;
    (t35246, t35253)
}
