//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1053/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1053<F: Float>(t204: F, t5026: F, t5070: F, t1503: F, t4952: F, t5335: F, t555: F, t1497: F, t1622: F, t4920: F, t4885: F, t496: F) -> (F, F, F, F) {
    let t16489 = F::cast_from(0.4274e0_f64) * t204 * t5070 * t5026;
    let t16493 = F::cast_from(0.69263436422725855036e2_f64) * t555 * t1503 * t4952 * t5335;
    let t16497 = F::cast_from(0.62337092780453269531e3_f64) * t555 * t4920 * t1497 * t1622;
    let t16502 = t496 * t4885;
    (t16489, t16493, t16497, t16502)
}
