//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1003/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1003<F: Float>(t251: F, t691: F, t1018: F, t86: F, t4996: F, t2822: F, t4989: F, t1131: F, t3209: F, t4904: F, t743: F, t4907: F) -> (F, F, F, F, F, F, F) {
    let t13396 = t691 * t251;
    let t13398 = t86 * t13396 * t1018;
    let t13399 = t13398 * t4996;
    let t13408 = t2822 * t4989;
    let t13409 = F::cast_from(0.22109259259259259258e-2_f64) * t13408;
    let t13410 = t3209 * t1131;
    let t13472 = F::cast_from(0.4705225e-4_f64) * t743 * t4904;
    let t13473 = t743 * t4907;
    (t13396, t13399, t13408, t13409, t13410, t13472, t13473)
}
