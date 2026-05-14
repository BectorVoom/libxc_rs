//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1298/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1298<F: Float>(t20: F, t32597: F, t1101: F, t9382: F, t9368: F, t111009: F, t32605: F, t15484: F, t9364: F, t15696: F, t273: F, t397: F, t43670: F, t43939: F, t32628: F, t32664: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t111028 = t32597 * t20;
    let t111029 = t1101 * t111028;
    let t111030 = t111029 * t9382;
    let t111032 = t111029 * t9368;
    let t111034 = t111009 * t9382;
    let t111036 = t32605 * t20;
    let t111037 = t1101 * t111036;
    let t111038 = t111037 * t9382;
    let t111040 = t111037 * t9368;
    let t111042 = t15484 * t9364;
    let t111043 = t111042 * t9382;
    let t111048 = t397 * t273 * t43670 * t15696;
    let t111049 = t43939 * t9364 * t111048;
    let t111051 = t32664 * t32628;
    (t111028, t111030, t111032, t111034, t111036, t111038, t111040, t111042, t111043, t111048, t111049, t111051)
}
