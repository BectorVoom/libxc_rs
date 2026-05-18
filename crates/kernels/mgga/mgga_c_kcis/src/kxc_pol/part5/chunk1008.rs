//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1008/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1008<F: Float>(t10114: F, t4840: F, t1027: F, t4852: F, t1663: F, t2323: F) -> (F, F, F) {
    let t13686 = t10114 * t4840;
    let t13689 = F::new(0.93706135855523581992e-2) * t1027 * t4852;
    let t13710 = t2323 * t1663;
    (t13686, t13689, t13710)
}
