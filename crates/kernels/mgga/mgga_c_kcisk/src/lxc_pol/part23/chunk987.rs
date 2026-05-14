//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 987/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk987<F: Float>(t1329: F, t13437: F, t14181: F, t14195: F, t19948: F, t19953: F, t19956: F, t19959: F, t19962: F, t19966: F, t19969: F, t19972: F, t3930: F, t2213: F, t3915: F, t415: F) -> (F, F) {
    let t19977 = -0.3684876543209876543e-3 * t19948 - 0.49745833333333333332e-2 * t19953 - 0.24872916666666666666e-2 * t19956 + 0.16581944444444444444e-2 * t19959 + 0.27636574074074074073e-2 * t19962 - 0.16581944444444444444e-2 * t14181 + 0.22109259259259259258e-2 * t14195 - 0.88437037037037037034e-2 * t19966 - 0.43134342e-1 * t13437 * t19969 - 0.386e0 * t19972 * t1329 - 0.223494e0 * t3930 * t19969;
    let t19984 = t2213 * t3915;
    let t19985 = t415 * t19984;
    (t19977, t19985)
}
