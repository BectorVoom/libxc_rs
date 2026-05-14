//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 826/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk826<F: Float>(t1464: F, t16758: F, t3722: F, t5756: F, t1395: F, t11776: F, t2012: F, t3728: F, t5761: F, t4158: F, t4992: F, t86: F, t1489: F, t167: F, t4163: F, t4162: F) -> (F, F, F, F, F, F, F, F, F) {
    let t16759 = t1464 * t16758;
    let t16761 = t5756 * t3722;
    let t16762 = t1395 * t16761;
    let t16763 = t1464 * t16762;
    let t16765 = t11776 * t2012;
    let t16766 = t1464 * t16765;
    let t16768 = t3728 * t5761;
    let t16769 = 0.22109259259259259258e-2 * t16768;
    let t16771 = t86 * t4992 * t4158;
    let t16772 = t167 * t1489;
    let t16773 = t4163 * t16772;
    let t16774 = t4162 * t16773;
    (t16759, t16761, t16763, t16766, t16768, t16769, t16771, t16773, t16774)
}
