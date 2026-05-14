//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1091/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1091<F: Float>(t17817: F, t96694: F, t1095: F, t709: F, t14075: F, t2441: F, t6035: F, t13863: F, t9652: F, t3817: F, t703: F, t13519: F, t6798: F, t27565: F, t4952: F, t24330: F, t27588: F, t6043: F) -> (F, F, F, F, F, F, F, F) {
    let t108738 = t17817 * t96694;
    let t108739 = t1095 * t709;
    let t108754 = t6035 * t2441 * t14075;
    let t108758 = t6035 * t9652 * t13863;
    let t108761 = t703 * t3817;
    let t108766 = t13519 * t6798;
    let t108773 = t27565 * t4952;
    let t108781 = 0.25537443351851851852e-1 * t6043 * t24330 * t27588;
    (t108738, t108739, t108754, t108758, t108761, t108766, t108773, t108781)
}
