//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 955/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk955<F: Float>(t1020: F, t13393: F, t251: F, t691: F, t1018: F, t86: F, t4996: F, t2855: F, t4621: F, t1021: F, t4994: F, t2825: F, t5005: F, t2822: F, t4989: F, t1131: F, t3209: F) -> (F, F, F, F, F, F, F, F) {
    let t13394 = t1020 * t13393;
    let t13396 = t691 * t251;
    let t13398 = t86 * t13396 * t1018;
    let t13399 = t13398 * t4996;
    let t13401 = t2855 * t4621;
    let t13402 = t1021 * t13401;
    let t13403 = t4994 * t13402;
    let t13405 = t2825 * t5005;
    let t13406 = t1020 * t13405;
    let t13408 = t2822 * t4989;
    let t13409 = 0.22109259259259259258e-2 * t13408;
    let t13410 = t3209 * t1131;
    (t13394, t13396, t13399, t13403, t13406, t13408, t13409, t13410)
}
