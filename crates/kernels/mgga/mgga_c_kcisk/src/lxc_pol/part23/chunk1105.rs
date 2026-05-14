//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1105/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1105<F: Float>(t167: F, t3139: F, t3241: F, t1001: F, t967: F, t1035: F, t3174: F, t206: F, t213: F, t2689: F, t3253: F, t3132: F, t9352: F, t1042: F, t9355: F, t3138: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t31831 = t167 * t3139;
    let t31832 = t3241 * t31831;
    let t31834 = t967 * t1001;
    let t31835 = t1035 * t31834;
    let t31837 = t167 * t3174;
    let t31838 = t1035 * t31837;
    let t31840 = t206 * t213;
    let t31842 = t3253 * t2689;
    let t31844 = t3132 * t9352;
    let t31846 = t1042 * t9355;
    let t31848 = t2689 * t3139;
    let t31849 = t3138 * t31848;
    (t31831, t31832, t31834, t31835, t31837, t31838, t31840, t31842, t31844, t31846, t31848, t31849)
}
