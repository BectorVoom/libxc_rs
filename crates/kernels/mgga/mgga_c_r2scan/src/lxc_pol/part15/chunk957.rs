//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 957/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk957<F: Float>(t2317: F, t3436: F, t158: F, t122: F, t166: F, t874: F, t3434: F, t502: F, t58: F, t875: F) -> (F, F, F, F, F) {
    let t10927 = t3436 * t2317;
    let t10928 = t10927 * t158;
    let t10929 = t166 * t122;
    let t10930 = t10929 * t874;
    let t10932 = t3434 * t10928 * t10930;
    let t10933 = F::new(0.43368970657079495312e-4) * t10932;
    let t10935 = t502 * t875 * t58;
    (t10928, t10929, t10930, t10933, t10935)
}
