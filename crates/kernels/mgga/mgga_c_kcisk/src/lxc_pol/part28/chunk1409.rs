//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1409/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1409<F: Float>(t112095: F, t9072: F, t1934: F, t8831: F, t9705: F, t4581: F, t9082: F, t33094: F, t35313: F, t654: F, t9015: F, t9709: F, t2575: F, t33120: F, t7312: F, t117385: F, t9972: F) -> (F, F, F, F, F, F, F) {
    let t122333 = t112095 * t9072;
    let t122336 = t1934 * t8831;
    let t122337 = t122336 * t9705;
    let t122339 = t4581 * t9082;
    let t122341 = t33094 * t35313;
    let t122343 = t9015 * t654;
    let t122344 = t122343 * t9709;
    let t122347 = t2575 * t33120 * t7312;
    let t122349 = t117385 * t9972;
    (t122333, t122337, t122339, t122341, t122344, t122347, t122349)
}
