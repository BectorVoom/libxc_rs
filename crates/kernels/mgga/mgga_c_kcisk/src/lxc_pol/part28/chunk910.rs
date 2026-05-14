//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 910/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk910<F: Float>(t682: F, t7028: F, t11401: F, t2372: F, t180: F, t4594: F, t479: F, t1887: F, t4597: F, t3521: F, t7052: F, t7057: F, t2522: F, t3517: F, t2518: F, t4663: F, t673: F) -> (F, F, F, F, F, F, F, F, F) {
    let t16826 = t7028 * t682;
    let t16839 = t11401 * t2372;
    let t16844 = t180 * t479 * t4594;
    let t16845 = t1887 * t4597;
    let t16863 = 0.19711289e-2 * t3521 * t7052;
    let t16865 = 0.26281718666666666666e-2 * t3521 * t7057;
    let t16879 = t3517 * t2522;
    let t16885 = t3517 * t2518;
    let t16887 = t673 * t4663;
    (t16826, t16839, t16844, t16845, t16863, t16865, t16879, t16885, t16887)
}
