//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 839/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk839<F: Float>(t29597: F, t5321: F, t2586: F, t8971: F, t1948: F, t2572: F, t9016: F, t28963: F, t719: F, t735: F, t11775: F, t7320: F, t9069: F, t2576: F, t9086: F, t2567: F, t9019: F) -> (F, F, F, F, F, F, F) {
    let t29598 = t5321 * t29597;
    let t29600 = t2586 * t8971;
    let t29601 = t1948 * t29600;
    let t29603 = t9016 * t2572;
    let t29605 = t719 * t28963;
    let t29606 = t735 * t29605;
    let t29607 = t11775 * t29606;
    let t29609 = t7320 * t9069;
    let t29611 = t2576 * t9086;
    let t29613 = t2567 * t9019;
    (t29598, t29601, t29603, t29607, t29609, t29611, t29613)
}
