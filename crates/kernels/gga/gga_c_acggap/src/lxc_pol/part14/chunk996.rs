//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 996/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk996<F: Float>(t30861: F, t8649: F, t4372: F, t7647: F, t1427: F, t1983: F, t34186: F, t7586: F, t1545: F, t30948: F, t1456: F, t7614: F) -> (F, F, F, F, F, F) {
    let t35240 = t30861 * t8649;
    let t35244 = t7647 * t4372;
    let t35246 = t1983 * t1427;
    let t35248 = t34186 * t7586 * t35246;
    let t35249 = F::new(0.42874018118069736972e-2) * t35248;
    let t35250 = t30948 * t1545;
    let t35251 = F::new(0.16006300097412701803e-1) * t35250;
    let t35258 = t7614 * t1456;
    (t35240, t35244, t35246, t35249, t35251, t35258)
}
