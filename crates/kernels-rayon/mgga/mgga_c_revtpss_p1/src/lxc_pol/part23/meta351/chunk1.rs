//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1659/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1659(t14671: f64, t14686: f64, t837: f64, t10777: f64, t10779: f64, t1548: f64, t10811: f64, t4447: f64, t10815: f64, t1561: f64, t2741: f64, t4426: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t14688 = t14686 * t14671 * t837;
    let t14690 = 0.25410001404642664112e-4_f64 * t10777 * t14688;
    let t14701 = t10779 * t1548 * t837;
    let t14703 = 0.10164000561857065645e-3_f64 * t10777 * t14701;
    let t14705 = 0.20007875121765877254e-2_f64 * t10811 * t4447;
    let t14712 = t10815 * t1561;
    let t14715 = 0.20007875121765877254e-2_f64 * t2741 * t4426;
    (t14688, t14690, t14701, t14703, t14705, t14712, t14715)
}
