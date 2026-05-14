//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1064/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1064<F: Float>(t222: F, t25312: F, t295: F, t167: F, t3233: F, t3236: F, t9345: F, t1032: F, t967: F, t3139: F, t3241: F, t1001: F, t1035: F, t3174: F, t206: F, t213: F, t2689: F, t3253: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t223 = t222 <= zeta_threshold;
    let t28192 = piecewise3(t223, 0.0, t25312);
    let t28193 = t295 * t28192;
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
    let t31842 = t3253 * t2689;
    (t28192, t28193, t31825, t31827, t31829, t31831, t31832, t31834, t31835, t31837, t31838, t31840, t31842)
}
