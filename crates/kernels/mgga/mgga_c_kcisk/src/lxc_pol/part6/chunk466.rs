//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 466/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk466<F: Float>(t167: F, t3532: F, t408: F, t1218: F, t411: F, t338: F, t389: F, t394: F, t123: F, t6: F) -> (F, F, F, F, F, F, F) {
    let t3891 = t167 * t3532;
    let t3923 = t408 * t408;
    let t3924 = F::new(1.0) / t3923;
    let t3929 = F::new(1.0) / t1218 / t411;
    let t3930 = t338 * t3929;
    let t3933 = t389 * t394;
    let t3934 = t123 * t6;
    (t3891, t3923, t3924, t3929, t3930, t3933, t3934)
}
