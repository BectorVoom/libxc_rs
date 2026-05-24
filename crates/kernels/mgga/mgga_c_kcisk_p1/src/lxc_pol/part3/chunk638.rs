//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 638/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk638<F: Float>(t41: F, t4143: F, t1849: F, t719: F, t4594: F, t704: F, t1336: F, t140: F, t4597: F, t1683: F, t4790: F, t4595: F, t708: F) -> (F, F, F, F, F, F) {
    let t6443 = t41 * t4143;
    let t6666 = t719 * t1849;
    let t6672 = t4594 * t704;
    let t6674 = t140 * t1336 * t6672;
    let t6675 = t719 * t4597;
    let t6880 = t4790 * t1683;
    let t7000 = t4595 * t708;
    (t6443, t6666, t6674, t6675, t6880, t7000)
}
