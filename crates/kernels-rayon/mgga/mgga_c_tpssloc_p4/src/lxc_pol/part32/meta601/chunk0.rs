//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1990/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1990(t80847: f64, t131: f64, t22791: f64, t9537: f64, t1338: f64, t225: f64, t236: f64, t1336: f64, t2690: f64, t6950: f64, t1369: f64, t22782: f64, t3777: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t80848 = 455.0_f64 / 1296.0_f64 * t80847;
    let t80853 = t22791 * t131 * t9537;
    let t80854 = t225 * t1338;
    let t80855 = t80854 * t236;
    let t80866 = t1336 * t6950 * t2690;
    let t80867 = t80866 * t1369;
    let t80869 = t3777 * t22782;
    (t80848, t80853, t80854, t80855, t80866, t80867, t80869)
}
