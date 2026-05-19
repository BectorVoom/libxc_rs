//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1001/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1001<F: Float>(t7433: F, t8739: F, t1089: F, t2079: F, t535: F, t7542: F, t1967: F, t8978: F, t31095: F, t31100: F, t33953: F, t5127: F) -> (F, F, F, F, F, F) {
    let t35260 = t7433 * t8739;
    let t35271 = t2079 * t1089 * t535 * t7542;
    let t35273 = t1967 * t8978;
    let t35278 = F::cast_from(0.17149607247227894789e-2_f64) * t31095;
    let t35279 = F::cast_from(0.42874018118069736972e-2_f64) * t31100;
    let t35284 = t33953 * t5127;
    (t35260, t35271, t35273, t35278, t35279, t35284)
}
