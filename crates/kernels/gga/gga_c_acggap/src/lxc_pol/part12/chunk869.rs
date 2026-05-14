//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 869/1092 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk869<F: Float>(t157: F, t309: F, t929: F, t1679: F, t811: F, t9460: F, t2248: F, t469: F, t301: F, t694: F, t1268: F, t8040: F, t10761: F, t467: F, t8034: F, t839: F) -> (F, F, F, F, F, F, F) {
    let t32199 = t309 * t929 * t157;
    let t32257 = t1679 * t9460 * t811;
    let t32262 = t2248 * t469;
    let t32264 = t694 * t32262 * t301;
    let t32276 = t1679 * t8040 * t1268;
    let t32283 = t1679 * t10761 * t467;
    let t32298 = t694 * t8034 * t839;
    (t32199, t32257, t32262, t32264, t32276, t32283, t32298)
}
