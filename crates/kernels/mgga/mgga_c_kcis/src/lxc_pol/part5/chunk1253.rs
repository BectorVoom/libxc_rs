//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1253/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1253<F: Float>(t1468: F, t20943: F, t1464: F, t5748: F, t5881: F, t5752: F, t5769: F, t1394: F, t5737: F, t5885: F, t1489: F, t7202: F) -> (F, F, F, F, F) {
    let t20944 = t1468 * t20943;
    let t20945 = t1464 * t20944;
    let t20947 = t5748 * t5881;
    let t20948 = t1464 * t20947;
    let t20950 = t5752 * t5769;
    let t20951 = t1394 * t20950;
    let t20953 = t5885 * t5737;
    let t20956 = t7202 * t1489;
    (t20945, t20948, t20951, t20953, t20956)
}
