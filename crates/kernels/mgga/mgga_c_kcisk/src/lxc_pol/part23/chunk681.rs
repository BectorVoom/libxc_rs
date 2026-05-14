//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 681/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk681<F: Float>(t3776: F, t6006: F, t1340: F, t1411: F, t2177: F, t3512: F, t1339: F, t1224: F, t2075: F, t4009: F, t4013: F, t5671: F, t1225: F, t5676: F, t416: F, t918: F) -> (F, F, F, F, F, F, F, F, F) {
    let t6007 = t3776 * t6006;
    let t6008 = t1340 * t6007;
    let t6009 = t1411 * t6008;
    let t6011 = t3512 * t2177;
    let t6012 = t1339 * t6011;
    let t6020 = t1224 * t4009 * t2075;
    let t6023 = t1224 * t4013 * t5671;
    let t6026 = t1224 * t1225 * t5676;
    let t6028 = t918 * t416;
    (t6007, t6008, t6009, t6011, t6012, t6020, t6023, t6026, t6028)
}
