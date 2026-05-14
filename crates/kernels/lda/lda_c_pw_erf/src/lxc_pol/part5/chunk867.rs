//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 867/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk867<F: Float>(t2055: F, t933: F, t1950: F, t925: F, t1945: F, t1953: F, t817: F, t1955: F, t8930: F, t1284: F, t4571: F, t3704: F, t4505: F, t12428: F, t1351: F, t3604: F, t4521: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t13663 = t933 * t2055;
    let t13710 = t925 * t1950;
    let t13714 = t925 * t1945;
    let t13715 = 0.03199259259259259 * t13714;
    let t13731 = t1953 * t817;
    let t13736 = t8930 * t1955;
    let t13749 = t1284 * t4571;
    let t13750 = 8.0 / 45.0 * t13749;
    let t13771 = t4505 * t3704;
    let t13797 = t12428 * t1351;
    let t13812 = t4521 * t3604;
    (t13663, t13710, t13714, t13715, t13731, t13736, t13750, t13771, t13797, t13812)
}
