//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1129/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1129<F: Float>(t142: F, t6293: F, t8888: F, t30120: F, t9649: F, t4680: F, t7413: F, t9648: F, t1815: F, t1983: F, t30127: F, t7586: F) -> (F, F, F, F) {
    let t39632 = t8888 * t142 * t6293;
    let t39640 = t30120 * t9649;
    let t39643 = t7413 * t4680 * t9648;
    let t39647 = t30127 * t7586 * t1983 * t1815;
    (t39632, t39640, t39643, t39647)
}
