//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 819/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk819<F: Float>(t3142: F, t743: F, t3145: F, t2635: F, t3160: F, t1072: F, t2630: F, t2844: F, t89: F, t740: F, t113: F, t9494: F) -> (F, F, F, F, F, F, F, F) {
    let t10056 = t743 * t3142;
    let t10058 = t743 * t3145;
    let t10087 = t3160 * t2635;
    let t10091 = t1072 * t2630;
    let t10093 = t89 * t2844;
    let t10096 = t740 * t2844;
    let t10097 = t10096 * t2630;
    let t10099 = t113 * t9494;
    (t10056, t10058, t10087, t10091, t10093, t10096, t10097, t10099)
}
