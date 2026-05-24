//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 936/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk936<F: Float>(t1301: F, t3981: F, t13614: F, t397: F, t403: F, t396: F, t12951: F, t12830: F, t3952: F, t12924: F, t1313: F, t1312: F) -> (F, F, F, F) {
    let t13868 = t1301 * t3981;
    let t13871 = t397 * t13614 * t403;
    let t13873 = F::cast_from(0.19989765240197019125e-1_f64) * t396 * t13871;
    let t13878 = t403 * t12951;
    let t13879 = t13878 * t12830;
    let t13880 = t3952 * t13879;
    let t13885 = t1313 * t12924;
    let t13886 = t1312 * t13885;
    (t13868, t13873, t13880, t13886)
}
