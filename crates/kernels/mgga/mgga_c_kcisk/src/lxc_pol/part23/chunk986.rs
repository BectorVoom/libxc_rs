//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 986/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk986<F: Float>(t1411: F, t19958: F, t3760: F, t5606: F, t3759: F, t1333: F, t5616: F, t13440: F, t2173: F, t3922: F, t1219: F, t5798: F) -> (F, F, F, F, F, F) {
    let t19959 = t1411 * t19958;
    let t19961 = t5606 * t3760;
    let t19962 = t3759 * t19961;
    let t19966 = t1333 * t5616;
    let t19968 = t2173 * t13440;
    let t19969 = t19968 * t3922;
    let t19972 = t5798 * t1219;
    (t19959, t19962, t19966, t19968, t19969, t19972)
}
