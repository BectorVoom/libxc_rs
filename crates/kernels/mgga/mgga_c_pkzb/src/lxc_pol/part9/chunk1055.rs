//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1055/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1055<F: Float>(t148: F, t1515: F, t1518: F, t204: F, t5026: F, t5070: F, t1503: F, t4952: F, t5335: F, t555: F, t1497: F, t1622: F, t4920: F) -> (F, F, F, F) {
    let t16486 = F::new(0.28493333333333333333e0) * t204 * t148 * t1515 * t1518;
    let t16489 = F::new(0.4274e0) * t204 * t5070 * t5026;
    let t16493 = F::new(0.69263436422725855036e2) * t555 * t1503 * t4952 * t5335;
    let t16497 = F::new(0.62337092780453269531e3) * t555 * t4920 * t1497 * t1622;
    (t16486, t16489, t16493, t16497)
}
