//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1106/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1106<F: Float>(t28164: F, t28176: F, t44: F, t167: F, t3233: F, t3236: F, t9345: F, t1032: F, t967: F, t3139: F, t3241: F, t1001: F, t1035: F, t3174: F, t206: F, t213: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t28178 = (t28164 + t28176) * t44;
    let t31825 = t3233 * t167;
    let t31827 = t3236 * t9345;
    let t31829 = t1032 * t967;
    let t31831 = t167 * t3139;
    let t31832 = t3241 * t31831;
    let t31834 = t967 * t1001;
    let t31835 = t1035 * t31834;
    let t31837 = t167 * t3174;
    let t31838 = t1035 * t31837;
    let t31840 = t206 * t213;
    (t28178, t31825, t31827, t31829, t31831, t31832, t31834, t31835, t31837, t31838, t31840)
}
