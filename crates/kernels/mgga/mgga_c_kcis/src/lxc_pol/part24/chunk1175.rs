//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1175/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1175<F: Float>(t283: F, t3177: F, t2193: F, t2196: F, t44682: F, t1086: F, t3225: F, t3245: F, t7727: F, t7735: F, t26972: F, t7780: F) -> (F, F, F, F, F, F) {
    let t92917 = t3177 * t283;
    let t92964 = F::new(0.12871334876543209877e-3) * t2193 * t44682 * t2196;
    let t92972 = t1086 * t3225;
    let t92993 = t3245 * t7727;
    let t92997 = t3245 * t7735;
    let t93016 = t7780 * t26972;
    (t92917, t92964, t92972, t92993, t92997, t93016)
}
