//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1109/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1109<F: Float>(t26654: F, t838: F, t26633: F, t26652: F, t26420: F, t12286: F, t491: F, t990: F, t3733: F, t27368: F, t61287: F, t16968: F, t3717: F, t1377: F, t1593: F, t1444: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t93826 = t838 * t26654;
    let t93848 = 3.0 * t26633;
    let t93849 = 3.0 * t26652;
    let t93852 = 12.0 * t26420;
    let t94208 = t12286 * t491 * t990;
    let t94216 = t3733 * t491;
    let t94227 = t27368 * t61287;
    let t94228 = t16968 * t3717;
    let t94246 = t1593 * t1377;
    let t94274 = t3717 * t1444;
    (t93826, t93848, t93849, t93852, t94208, t94216, t94227, t94228, t94246, t94274)
}
