//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1045/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1045<F: Float>(t2030: F, t4586: F, t7815: F, t7447: F, t8800: F, t30219: F, t8661: F, t7575: F, t7576: F, t8480: F, t30543: F, t8446: F) -> (F, F, F, F, F) {
    let t34604 = t2030 * t7815 * t4586;
    let t34609 = t7447 * t8800;
    let t34611 = t30219 * t8661;
    let t34614 = t7575 * t8480 * t7576;
    let t34616 = t30543 * t8446;
    (t34604, t34609, t34611, t34614, t34616)
}
