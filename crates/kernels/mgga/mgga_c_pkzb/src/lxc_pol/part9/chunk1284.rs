//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1284/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1284<F: Float>(t6337: F, t8012: F, t898: F, t1208: F, t18520: F, t6283: F, t2328: F, t8289: F, t8293: F, t6324: F, t8287: F, t3135: F, t6230: F, t8288: F) -> (F, F, F, F, F, F) {
    let t22511 = F::new(0.51947577317044391277e2) * t898 * t8012 * t6337;
    let t22515 = F::new(0.12304822629859687989e5) * t898 * t18520 * t1208 * t6283;
    let t22517 = F::new(0.30762056574649219973e4) * t2328 * t8289;
    let t22519 = F::new(0.51947577317044391277e2) * t2328 * t8293;
    let t22522 = F::new(0.6233709278045326953e3) * t898 * t8287 * t6324;
    let t22526 = F::new(0.30762056574649219973e4) * t898 * t6230 * t3135 * t8288;
    (t22511, t22515, t22517, t22519, t22522, t22526)
}
