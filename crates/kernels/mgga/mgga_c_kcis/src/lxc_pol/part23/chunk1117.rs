//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1117/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1117<F: Float>(t16789: F, t51692: F, t7923: F, t27357: F, t5721: F, t94228: F, t491: F, t5747: F, t1394: F, t4165: F, t28499: F, t4173: F, t16937: F, t28484: F, t27369: F, t16941: F, t28494: F, t7908: F) -> (F, F, F, F, F, F, F, F) {
    let t98463 = t51692 * t7923 * t16789;
    let t98466 = t94228 * t5721 * t27357;
    let t98470 = t5747 * t491;
    let t98472 = t1394 * t98470 * t4165;
    let t98475 = t1394 * t28499 * t4173;
    let t98487 = t16937 * t28484;
    let t98489 = 0.20612155671296296296e-4 * t27369 * t98487;
    let t98491 = t7908 * t16941 * t28494;
    (t98463, t98466, t98470, t98472, t98475, t98487, t98489, t98491)
}
