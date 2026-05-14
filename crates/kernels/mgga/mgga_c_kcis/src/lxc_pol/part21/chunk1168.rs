//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1168/1221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1168<F: Float>(t1020: F, t26671: F, t4796: F, t27864: F, t2822: F, t13440: F, t27763: F, t3205: F, t95664: F, t27836: F, t3213: F, t3245: F, t8057: F, t27936: F, t7699: F, t13420: F, t3200: F, t92808: F) -> (F, F, F, F, F, F, F, F, F) {
    let t96399 = t1020 * t26671 * t4796;
    let t96401 = t2822 * t27864;
    let t96402 = 0.22109259259259259258e-2 * t96401;
    let t96404 = t1020 * t27763 * t13440;
    let t96407 = t1020 * t95664 * t3205;
    let t96410 = t1020 * t27836 * t3213;
    let t96412 = t3245 * t8057;
    let t96418 = 0.46336805555555555556e-3 * t27936 * t7699;
    let t96420 = t3200 * t92808 * t13420;
    (t96399, t96401, t96402, t96404, t96407, t96410, t96412, t96418, t96420)
}
