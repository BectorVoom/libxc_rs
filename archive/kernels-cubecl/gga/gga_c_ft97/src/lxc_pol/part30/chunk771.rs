//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 771/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk771<F: Float>(t2567: F, t7546: F, t684: F, t2606: F, t1882: F, t7548: F, t713: F, t7553: F, t729: F, t762: F, t258: F, t7440: F) -> (F, F, F, F, F, F, F) {
    let t33759 = t2567 * t7546;
    let t33760 = t33759 * t684;
    let t33761 = t2606 * t33760;
    let t33765 = F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1882 * t7548;
    let t33766 = t7553 * t713;
    let t33768 = t729 * t762 * t33766;
    let t33771 = t258 * t7440;
    (t33759, t33760, t33761, t33765, t33766, t33768, t33771)
}
