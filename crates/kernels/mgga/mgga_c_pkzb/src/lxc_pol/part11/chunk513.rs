//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 513/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk513<F: Float>(t158: F, t2612: F, t2623: F, t133: F, t160: F, t1020: F, t614: F, t568: F, t2575: F, t596: F, t1029: F, t1031: F, t162: F, t594: F, t597: F) -> (F, F, F, F, F, F) {
    let t2625 = (t2612 + t2623) * t158;
    let t2631 = t160 * t133;
    let t2632 = t614 * t1020;
    let t2633 = t2632 * t568;
    let t2636 = t596 * t2575;
    let t2639 = F::cast_from(3.0_f64) * t1029 * t597 + F::cast_from(3.0_f64) * t1031 * t594 + F::cast_from(3.0_f64) * t160 * t2636 - t162 * t2625 - F::cast_from(12.0_f64) * t2631 * t2633;
    (t2625, t2631, t2632, t2633, t2636, t2639)
}
