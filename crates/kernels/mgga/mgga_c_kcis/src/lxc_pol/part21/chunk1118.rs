//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1118/1221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1118<F: Float>(t3452: F, t5026: F, t1817: F, t9568: F, t8072: F, t92415: F, t1189: F, t13106: F, t14853: F, t26930: F, t14839: F, t7754: F, t3348: F, t4999: F, t14703: F, t26896: F) -> (F, F, F, F, F, F, F, F) {
    let t95432 = t5026 * t3452;
    let t95434 = t9568 * t1817;
    let t95436 = t92415 * t8072;
    let t95438 = t13106 * t1189;
    let t95440 = t26930 * t14853;
    let t95442 = t7754 * t14839;
    let t95444 = t4999 * t3348;
    let t95446 = t26896 * t14703;
    (t95432, t95434, t95436, t95438, t95440, t95442, t95444, t95446)
}
