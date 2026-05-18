//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1252/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1252<F: Float>(t98743: F, t2237: F, t54162: F, t8158: F, t28453: F, t4142: F, t3245: F, t8171: F, t8179: F, t12147: F, t28550: F, t7908: F) -> (F, F, F, F, F, F, F) {
    let t98744 = F::new(0.22109259259259259258e-2) * t98743;
    let t98777 = t2237 * t54162 * t8158;
    let t98794 = t4142 * t28453;
    let t98795 = F::new(0.14739506172839506172e-2) * t98794;
    let t98804 = t3245 * t8171;
    let t98806 = t3245 * t8179;
    let t98813 = F::new(0.15445601851851851852e-3) * t7908 * t12147 * t28550;
    (t98744, t98777, t98794, t98795, t98804, t98806, t98813)
}
