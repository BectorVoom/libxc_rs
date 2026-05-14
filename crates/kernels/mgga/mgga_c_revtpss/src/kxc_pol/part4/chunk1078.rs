//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1078/1274 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk1078<F: Float>(t14685: F, t220: F, t14671: F, t837: F, t10777: F, t125: F, t4343: F, t2747: F, t2646: F, t4450: F, t10779: F, t1548: F, t10811: F, t4447: F, t14676: F, t2749: F) -> (F, F, F, F, F, F, F) {
    let t14686 = t14685 * t220;
    let t14688 = t14686 * t14671 * t837;
    let t14690 = 0.25410001404642664112e-4 * t10777 * t14688;
    let t14691 = t125 * t4343;
    let t14693 = t2747 * t14691 * t837;
    let t14697 = t2747 * t4450 * t2646;
    let t14701 = t10779 * t1548 * t837;
    let t14703 = 0.10164000561857065645e-3 * t10777 * t14701;
    let t14705 = 0.20007875121765877254e-2 * t10811 * t4447;
    let t14707 = t2747 * t14676 * t2749;
    (t14686, t14690, t14693, t14697, t14703, t14705, t14707)
}
