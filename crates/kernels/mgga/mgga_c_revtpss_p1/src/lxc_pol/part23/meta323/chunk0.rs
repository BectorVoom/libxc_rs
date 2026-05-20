//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1612/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1612<F: Float>(t13731: F, t2782: F, t212: F, t5710: F, t1358: F, t689: F, t221: F, t3979: F, t5591: F, t3978: F, t3989: F, t5614: F) -> (F, F, F, F, F, F, F) {
    let t13733 = F::cast_from(0.21951497276451705328e-1_f64) * t2782 * t13731;
    let t13734 = t212 * t5710;
    let t13735 = t13734 * t1358;
    let t13737 = F::cast_from(0.10975748638225852664e-1_f64) * t689 * t13735;
    let t13760 = t3979 * t221 * t5591;
    let t13762 = F::cast_from(0.10164000561857065645e-3_f64) * t3978 * t13760;
    let t13763 = t3989 * t5614;
    (t13733, t13734, t13735, t13737, t13760, t13762, t13763)
}
