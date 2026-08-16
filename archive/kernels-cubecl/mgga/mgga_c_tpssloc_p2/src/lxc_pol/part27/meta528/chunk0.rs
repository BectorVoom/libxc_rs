//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1938/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1938<F: Float>(t343: F, t381: F, t6690: F, t25712: F, t4347: F, t6689: F, t7561: F, t968: F, t1920: F, t1625: F, t6688: F, t6691: F) -> (F, F, F, F, F, F, F, F, F) {
    let t25796 = t343 * t381;
    let t25797 = t25796 * t6690;
    let t25798 = t25712 * t25797;
    let t25801 = t6690 * t4347;
    let t25802 = t6689 * t25801;
    let t25806 = t968 * t7561;
    let t25807 = t1920 * t25806;
    let t25810 = t6688 * t1625;
    let t25811 = t25810 * t6691;
    (t25796, t25797, t25798, t25801, t25802, t25806, t25807, t25810, t25811)
}
