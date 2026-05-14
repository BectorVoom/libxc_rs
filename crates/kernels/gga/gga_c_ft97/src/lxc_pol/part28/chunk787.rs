//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 787/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk787<F: Float>(t34947: F, t605: F, t12664: F, t7400: F, t5935: F, t6718: F, t1053: F, t32729: F, t1384: F, t26590: F, t34918: F, t525: F, t165: F, t28: F, t1058: F, t7340: F) -> (F, F, F, F, F, F, F, F, F) {
    let t34948 = t605 * t34947;
    let t34950 = t12664 * t7400;
    let t34952 = t5935 * t6718;
    let t34954 = t32729 * t1053;
    let t34956 = t26590 * t1384;
    let t34961 = t525 * t34918;
    let t34962 = t34961 * t165;
    let t34963 = t28 * t34962;
    let t34966 = t7340 * t1058;
    (t34948, t34950, t34952, t34954, t34956, t34961, t34962, t34963, t34966)
}
