//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 838/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk838<F: Float>(t1466: F, t5869: F, t12274: F, t2013: F, t3728: F, t5761: F, t4158: F, t4992: F, t86: F, t5659: F, t11913: F, t5668: F, sigma2: F) -> (F, F, F, F, F, F, F, F) {
    let t16751 = t5869 * t1466;
    let t16752 = t16751 * sigma2;
    let t16756 = t12274 * t2013;
    let t16768 = t3728 * t5761;
    let t16769 = F::cast_from(0.22109259259259259258e-2_f64) * t16768;
    let t16771 = t86 * t4992 * t4158;
    let t16788 = t86 * t4992 * t5659;
    let t16793 = t11913 * t5668;
    (t16751, t16752, t16756, t16768, t16769, t16771, t16788, t16793)
}
