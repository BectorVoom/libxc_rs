//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1126/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1126<F: Float>(t10568: F, t1775: F, t2739: F, t505: F, t11176: F, t303: F, t10607: F, t10362: F, t289: F, t287: F, t2726: F, t2735: F) -> (F, F, F, F, F, F, F) {
    let t43563 = t1775 * t10568;
    let t43568 = t505 * t2739;
    let t43574 = F::new(280.0) / F::new(81.0) * t11176 * t303;
    let t43578 = t1775 * t10607;
    let t43585 = F::new(1.0) / t10362 / t289;
    let t43586 = t287 * t43585;
    let t43587 = t2726 * t2726;
    let t43595 = t2735 * t2735;
    (t43563, t43568, t43574, t43578, t43586, t43587, t43595)
}
