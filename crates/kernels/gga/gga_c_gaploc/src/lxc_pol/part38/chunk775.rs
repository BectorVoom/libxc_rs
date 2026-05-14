//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 775/861 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk775<F: Float>(t313: F, t45001: F, t11801: F, t41105: F, t1445: F, t2639: F, t43425: F, t43435: F, t44967: F, t45066: F, t45212: F, t45215: F, t45217: F, t45219: F, t45222: F, t45226: F, t45229: F, t45232: F, t45234: F, t45238: F, t45242: F, t45243: F, t45247: F, t45251: F, t833: F) -> (F,) {
    let t45252 = t313 * t45001;
    let t45256 = 0.42900587942220512003e1 * t11801 * t41105;
    let t45257 = 0.23005755572352449806e2 * t833 * t1445 * t44967 + 0.23005755572352449806e2 * t833 * t1445 * t45066 + t45212 + t45215 + t45217 - 0.76685851907841499354e0 * t45219 - 0.76685851907841499354e0 * t45222 + t45226 - t45229 - t45232 + 0.57514388930881124515e0 * t45234 - 0.85206502119823888171e0 * t43425 - 0.51762950037793012064e1 * t45238 + t45242 - t45243 - 0.15337170381568299871e1 * t43435 + t45247 - t45251 - 0.10725146985555128001e1 * t45252 * t2639 + t45256;
    (t45257,)
}
