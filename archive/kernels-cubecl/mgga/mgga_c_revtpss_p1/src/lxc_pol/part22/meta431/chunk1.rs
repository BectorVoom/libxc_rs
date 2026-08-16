//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2057/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2057<F: Float>(t14671: F, t14686: F, t837: F, t10777: F, t125: F, t4343: F, t2747: F, t2646: F, t4450: F, t10779: F, t1548: F, t10811: F, t4447: F) -> (F, F, F, F, F, F, F) {
    let t14688 = t14686 * t14671 * t837;
    let t14690 = F::cast_from(0.25410001404642664112e-4_f64) * t10777 * t14688;
    let t14691 = t125 * t4343;
    let t14693 = t2747 * t14691 * t837;
    let t14697 = t2747 * t4450 * t2646;
    let t14701 = t10779 * t1548 * t837;
    let t14703 = F::cast_from(0.10164000561857065645e-3_f64) * t10777 * t14701;
    let t14705 = F::cast_from(0.20007875121765877254e-2_f64) * t10811 * t4447;
    (t14688, t14690, t14693, t14697, t14701, t14703, t14705)
}
