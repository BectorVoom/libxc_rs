//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2021/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2021<F: Float>(t154: F, t8705: F, t1887: F, t534: F, t12267: F, t6951: F, t131: F, t22791: F, t9537: F, t1338: F, t225: F, t236: F) -> (F, F, F, F, F, F) {
    let t80845 = t8705 * t154;
    let t80847 = t80845 * t534 * t1887;
    let t80848 = F::cast_from(455.0_f64) / F::cast_from(1296.0_f64) * t80847;
    let t80849 = t12267 * t6951;
    let t80853 = t22791 * t131 * t9537;
    let t80854 = t225 * t1338;
    let t80855 = t80854 * t236;
    (t80845, t80848, t80849, t80853, t80854, t80855)
}
