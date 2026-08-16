//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 854/1255 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk854<F: Float>(t225: F, t5262: F, t5270: F, t546: F, t68: F, t1365: F, t1799: F, t1307: F, t1347: F, t5187: F, t1345: F, t1348: F, t1819: F, t1821: F, t548: F) -> (F, F, F, F, F, F) {
    let t5272 = (t5262 + t5270) * t225;
    let t5278 = t546 * t68;
    let t5279 = t1365 * t1799;
    let t5280 = t5279 * t1307;
    let t5283 = t1347 * t5187;
    let t5286 = F::cast_from(3.0_f64) * t1345 * t1821 + F::cast_from(3.0_f64) * t1348 * t1819 - t5272 * t548 - F::cast_from(12.0_f64) * t5278 * t5280 + F::cast_from(3.0_f64) * t5283 * t546;
    (t5272, t5278, t5279, t5280, t5283, t5286)
}
