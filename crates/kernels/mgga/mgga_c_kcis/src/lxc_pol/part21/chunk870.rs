//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 870/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk870<F: Float>(t1021: F, t13401: F, t4994: F, t2825: F, t5005: F, t1020: F, t2822: F, t4989: F, t1131: F, t3209: F, t3212: F, t4580: F) -> (F, F, F, F, F, F) {
    let t13402 = t1021 * t13401;
    let t13403 = t4994 * t13402;
    let t13405 = t2825 * t5005;
    let t13406 = t1020 * t13405;
    let t13408 = t2822 * t4989;
    let t13409 = F::new(0.22109259259259259258e-2) * t13408;
    let t13410 = t3209 * t1131;
    let t13411 = t4580 * t3212;
    (t13403, t13406, t13408, t13409, t13410, t13411)
}
