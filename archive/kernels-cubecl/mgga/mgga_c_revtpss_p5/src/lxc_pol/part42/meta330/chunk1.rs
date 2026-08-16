//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1124/1505 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1124<F: Float>(t14685: F, t220: F, t14671: F, t837: F, t10777: F, t10779: F, t1548: F, t10811: F, t4447: F, t10815: F, t1561: F, t2741: F, t4426: F) -> (F, F, F, F, F, F) {
    let t14686 = t14685 * t220;
    let t14688 = t14686 * t14671 * t837;
    let t14690 = F::cast_from(0.25410001404642664112e-4_f64) * t10777 * t14688;
    let t14701 = t10779 * t1548 * t837;
    let t14703 = F::cast_from(0.10164000561857065645e-3_f64) * t10777 * t14701;
    let t14705 = F::cast_from(0.20007875121765877254e-2_f64) * t10811 * t4447;
    let t14712 = t10815 * t1561;
    let t14715 = F::cast_from(0.20007875121765877254e-2_f64) * t2741 * t4426;
    (t14686, t14690, t14703, t14705, t14712, t14715)
}
