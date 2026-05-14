//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1026/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1026<F: Float>(t25450: F, t6287: F, t25432: F, t25437: F, t6279: F, t140: F, t299: F, t8227: F, t1337: F, t2209: F, t5676: F, t3529: F, t5671: F, t25441: F, t25446: F, t21113: F, t25465: F) -> (F, F, F, F, F, F, F, F, F) {
    let t27345 = t6287 * t25450;
    let t27348 = t6287 * t25432;
    let t27351 = t6279 * t25437;
    let t27355 = t140 * t299 * t8227;
    let t27357 = t1337 * t2209;
    let t27358 = t27357 * t5676;
    let t27361 = t3529 * t2209;
    let t27362 = t27361 * t5671;
    let t27365 = t6287 * t25441;
    let t27371 = t6279 * t25446;
    let t27374 = t21113 * t25465;
    (t27345, t27348, t27351, t27355, t27358, t27362, t27365, t27371, t27374)
}
