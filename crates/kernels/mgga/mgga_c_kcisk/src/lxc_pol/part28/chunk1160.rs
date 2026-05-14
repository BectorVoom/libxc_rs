//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1160/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1160<F: Float>(t2041: F, t9758: F, t2707: F, t5579: F, t2068: F, t9406: F, t9639: F, t9904: F, t6650: F, t806: F, t2776: F, t1628: F, t2670: F, t1790: F, t2063: F, t33032: F) -> (F, F, F, F, F, F, F, F, F) {
    let t33306 = t9758 * t2041;
    let t33330 = t5579 * t2707;
    let t33986 = t2068 * t9406;
    let t34003 = t9904 * t9639;
    let t34005 = t6650 * t806;
    let t34006 = t2776 * t34005;
    let t34008 = t1628 * t2670;
    let t34009 = t2776 * t34008;
    let t34011 = t2063 * t1790;
    let t34012 = t33032 * t34011;
    (t33306, t33330, t33986, t34003, t34005, t34006, t34008, t34009, t34012)
}
