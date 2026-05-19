//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1241/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1241<F: Float>(t98254: F, t1394: F, t27364: F, t5649: F, t17270: F, t7923: F, t16778: F, t5780: F, t2242: F, t4134: F, t16634: F, t4160: F) -> (F, F, F, F, F) {
    let t98255 = F::cast_from(0.3684876543209876543e-2_f64) * t98254;
    let t98257 = t1394 * t27364 * t5649;
    let t98260 = t1394 * t7923 * t17270;
    let t98263 = t5780 * t7923 * t16778;
    let t98266 = t2242 * t4134;
    let t98268 = t4160 * t98266 * t16634;
    (t98255, t98257, t98260, t98263, t98268)
}
