//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 947/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk947<F: Float>(t7447: F, t8800: F, t30219: F, t8661: F, t7575: F, t7576: F, t8480: F, t30543: F, t8446: F, t30934: F, t8450: F, t30937: F, t8597: F, t8602: F, t1165: F, t4718: F, t7351: F, t7426: F) -> (F, F, F, F, F, F, F, F) {
    let t34609 = t7447 * t8800;
    let t34610 = 11.0 / 192.0 * t34609;
    let t34611 = t30219 * t8661;
    let t34612 = 0.47172138434406228102e-2 * t34611;
    let t34614 = t7575 * t8480 * t7576;
    let t34616 = t30543 * t8446;
    let t34617 = 0.18868855373762491241e-1 * t34616;
    let t34618 = t30934 * t8450;
    let t34620 = t30937 * t8597;
    let t34621 = 0.18868855373762491241e-2 * t34620;
    let t34622 = t30937 * t8602;
    let t34623 = 0.37737710747524982482e-2 * t34622;
    let t34626 = t7426 * t1165 * t7351 * t4718;
    (t34610, t34612, t34614, t34617, t34618, t34621, t34623, t34626)
}
