//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 997/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk997<F: Float>(t35258: F, t7433: F, t8739: F, t1089: F, t2079: F, t535: F, t7542: F, t1967: F, t8978: F, t33953: F, t5127: F, t13287: F, t31057: F) -> (F, F, F, F, F, F) {
    let t35259 = F::new(0.16006300097412701803e-1) * t35258;
    let t35260 = t7433 * t8739;
    let t35261 = F::new(0.37737710747524982482e-2) * t35260;
    let t35271 = t2079 * t1089 * t535 * t7542;
    let t35273 = t1967 * t8978;
    let t35274 = F::new(0.25724410870841842184e-2) * t35273;
    let t35284 = t33953 * t5127;
    let t35286 = t31057 * t13287 * t35284;
    (t35259, t35261, t35271, t35274, t35284, t35286)
}
