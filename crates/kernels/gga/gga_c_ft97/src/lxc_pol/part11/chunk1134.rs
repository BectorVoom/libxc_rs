//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1134/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1134<F: Float>(t10611: F, t458: F, t10572: F, t2: F, t33828: F, t1771: F, t2787: F, t10563: F, t1775: F, t798: F, t9567: F, t10613: F, t14961: F, t192: F, t2771: F, t41454: F, t41464: F, t41473: F, t41490: F, t4199: F, t4206: F, t42145: F, t43359: F, t43420: F, t43428: F, t43469: F, t43525: F, t462: F, t848: F, t92: F) -> (F, F) {
    let t43799 = t458 * t10611;
    let t43801 = t458 * t10572;
    let t43803 = t33828 * t2;
    let t43808 = t1771 * t2787;
    let t43831 = t1775 * t10563;
    let t43833 = t9567 * t798;
    let t43834 = t43833 * t2;
    let t43841 = F::new(4.0) / F::new(3.0) * t43799 + F::new(8.0) * t43801 + F::new(24.0) * t92 * t192 * t43803 * t43525 - F::new(8.0) / F::new(3.0) * t43808 - F::new(16.0) / F::new(3.0) * t462 * t10613 * t43359 + F::new(8.0) * t462 * t4199 * t41464 - F::new(20.0) / F::new(9.0) * t462 * t14961 * t41454 + F::new(4.0) / F::new(3.0) * t462 * t2771 * t43428 + F::new(8.0) / F::new(3.0) * t462 * t4206 * t41490 - F::new(8.0) / F::new(9.0) * t462 * t4199 * t41473 + F::new(4.0) / F::new(3.0) * t462 * t10613 * t43420 + F::new(8.0) / F::new(9.0) * t43831 + F::new(40.0) / F::new(27.0) * t462 * t43834 * t43469 - t462 * t848 * t42145 / F::new(3.0);
    (t43833, t43841)
}
