//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 250/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk250<F: Float>(t1162: F, t1165: F, t321: F, t317: F, t305: F, t306: F) -> (F, F, F, F, F, F) {
    let t1167 = -t1162 - F::new(0.17808333333333333333e-1) * t1165;
    let t1169 = F::new(0.62182e-1) * t1167 * t321;
    let t1170 = t317 * t317;
    let t1171 = F::new(1.0) / t1170;
    let t1172 = t305 * t1171;
    let t1173 = F::new(1.0) / t306;
    (t1167, t1169, t1170, t1171, t1172, t1173)
}
