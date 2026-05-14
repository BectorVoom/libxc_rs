//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 953/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk953<F: Float>(t1919: F, t695: F, t709: F, t2518: F, t3517: F, t4663: F, t673: F, t4624: F, t7029: F, t1814: F, t2372: F, t4658: F, t4629: F, t11313: F, t2514: F, t3521: F, t7031: F) -> (F, F, F, F, F, F, F, F) {
    let t16882 = t1919 * t709 * t695;
    let t16885 = t3517 * t2518;
    let t16887 = t673 * t4663;
    let t16888 = t7029 * t4624;
    let t16889 = t16887 * t16888;
    let t16892 = t1814 * t2372;
    let t16893 = t16892 * t4658;
    let t16894 = t4629 * t16893;
    let t16897 = t11313 * t2514;
    let t16900 = 0.98556445e-3 * t3521 * t7031;
    (t16882, t16885, t16888, t16889, t16893, t16894, t16897, t16900)
}
