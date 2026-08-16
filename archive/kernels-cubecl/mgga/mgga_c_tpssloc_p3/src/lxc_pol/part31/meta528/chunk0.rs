//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1742/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1742<F: Float>(t1824: F, t5318: F, t1372: F, t6387: F, t6414: F, t19731: F, t562: F, t20063: F, t3701: F, t1484: F, t2752: F, t17083: F, t225: F) -> (F, F, F, F, F, F, F) {
    let t57545 = t5318 * t1824;
    let t57607 = t1372 * t6387;
    let t57618 = t1372 * t6414;
    let t57704 = t562 * t19731;
    let t57806 = t20063 * t3701;
    let t57911 = t2752 * t1484;
    let t58143 = t17083 * t225;
    (t57545, t57607, t57618, t57704, t57806, t57911, t58143)
}
