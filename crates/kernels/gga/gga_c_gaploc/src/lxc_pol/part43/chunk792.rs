//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 792/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk792<F: Float>(t1445: F, t43213: F, t833: F, t43217: F, t13136: F, t2197: F, t10040: F, t25198: F, t41133: F, t13055: F, t5640: F, t13058: F, t1991: F, t20671: F, t28309: F, t33601: F) -> (F, F, F, F, F, F, F, F) {
    let t43636 = 0.11502877786176224903e2 * t833 * t1445 * t43213;
    let t43640 = 0.11502877786176224903e2 * t833 * t1445 * t43217;
    let t43645 = 0.11502877786176224903e2 * t2197 * t13136;
    let t43646 = t25198 * t10040;
    let t43648 = 0.19171462976960374838e1 * t41133;
    let t43652 = t5640 * t13055;
    let t43653 = 0.15337170381568299871e1 * t43652;
    let t43657 = t1991 * t13058;
    let t43658 = 0.1022478025437886658e1 * t43657;
    let t43660 = t28309 * t20671 * t33601;
    (t43636, t43640, t43645, t43646, t43648, t43653, t43658, t43660)
}
