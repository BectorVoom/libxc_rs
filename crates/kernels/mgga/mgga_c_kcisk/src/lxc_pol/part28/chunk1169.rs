//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1169/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1169<F: Float>(t1333: F, t9952: F, t9660: F, t9932: F, t1894: F, t2571: F, t33017: F, t1869: F, t1757: F, t2442: F, t9679: F, t2454: F, t739: F) -> (F, F, F, F, F, F, F, F, F) {
    let t34081 = t1333 * t9952;
    let t34083 = t9932 * t9660;
    let t34085 = t2571 * t1894;
    let t34086 = t33017 * t34085;
    let t34087 = t1869 * t34086;
    let t34089 = t2442 * t1757;
    let t34090 = t9679 * t34089;
    let t34091 = t1869 * t34090;
    let t34093 = t739 * t2454;
    (t34081, t34083, t34085, t34086, t34087, t34089, t34090, t34091, t34093)
}
