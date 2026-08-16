//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1112/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1112<F: Float>(t1456: F, t7614: F, t7433: F, t8739: F, t1181: F, t2068: F, t21342: F, t604: F, t1089: F, t2079: F, t535: F, t7542: F) -> (F, F, F, F) {
    let t35258 = t7614 * t1456;
    let t35259 = F::cast_from(0.16006300097412701803e-1_f64) * t35258;
    let t35260 = t7433 * t8739;
    let t35261 = F::cast_from(0.37737710747524982482e-2_f64) * t35260;
    let t35264 = t2068 * t1181 * t604 * t21342;
    let t35271 = t2079 * t1089 * t535 * t7542;
    (t35259, t35261, t35264, t35271)
}
