//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 1010/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk1010<F: Float>(t1312: F, t14892: F, t1591: F, t3283: F, t4400: F, t13456: F, t4406: F, t4391: F, t3952: F, t1588: F, t3532: F, t3278: F) -> (F, F, F, F, F, F) {
    let t14893 = t1312 * t14892;
    let t14896 = t3283 * t1591;
    let t14897 = t4400 * t14896;
    let t14898 = t1312 * t14897;
    let t14901 = t4406 * t13456;
    let t14902 = t1312 * t14901;
    let t14905 = t4391 * t13456;
    let t14906 = t3952 * t14905;
    let t14909 = t1588 * t3532;
    let t14910 = t3278 * t1591;
    (t14893, t14898, t14902, t14906, t14909, t14910)
}
