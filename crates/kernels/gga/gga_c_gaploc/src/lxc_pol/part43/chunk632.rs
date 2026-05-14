//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 632/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk632<F: Float>(t13118: F, t6111: F, t12709: F, t2949: F, t3234: F, t1445: F, t813: F, t13023: F, t833: F, t3040: F, t3267: F, t10012: F, t1022: F, t9438: F, t2684: F, t10007: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t13119 = t6111 * t13118;
    let t13120 = 0.59584149919750711116e-1 * t13119;
    let t13124 = 0.19171462976960374838e1 * t12709;
    let t13129 = t2949 * t3234;
    let t13130 = t1445 * t13129;
    let t13132 = 0.46011511144704899612e1 * t813 * t13130;
    let t13136 = t1445 * t13023;
    let t13138 = 0.11502877786176224903e2 * t833 * t13136;
    let t13140 = 0.35750489951850426669e0 * t3267 * t3040;
    let t13141 = t10012 * t1022;
    let t13142 = t9438 * t13141;
    let t13143 = t2684 * t13142;
    let t13144 = 0.15976219147466979032e-1 * t13143;
    let t13149 = t10007 * t1022;
    (t13120, t13124, t13129, t13130, t13132, t13136, t13138, t13140, t13141, t13142, t13144, t13149)
}
