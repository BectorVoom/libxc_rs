//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1974/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1974<F: Float>(t80847: F, t131: F, t22791: F, t9537: F, t1338: F, t225: F, t236: F, t1336: F, t2690: F, t6950: F, t1369: F, t22782: F, t3777: F) -> (F, F, F, F, F, F, F) {
    let t80848 = F::cast_from(455.0_f64) / F::cast_from(1296.0_f64) * t80847;
    let t80853 = t22791 * t131 * t9537;
    let t80854 = t225 * t1338;
    let t80855 = t80854 * t236;
    let t80866 = t1336 * t6950 * t2690;
    let t80867 = t80866 * t1369;
    let t80869 = t3777 * t22782;
    (t80848, t80853, t80854, t80855, t80866, t80867, t80869)
}
