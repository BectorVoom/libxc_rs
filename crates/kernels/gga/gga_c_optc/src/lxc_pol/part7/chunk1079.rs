//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1079/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1079<F: Float>(t6569: F, t732: F, t737: F, t2193: F, t104: F, t1975: F, t22497: F, t22562: F, t22578: F, t22581: F, t22593: F, t22685: F, t22687: F, t22690: F, t22694: F, t22697: F, t95: F) -> (F, F, F) {
    let t23413 = F::new(1820.0) / F::new(27.0) * t732 * t6569;
    let t23414 = t737 * t6569;
    let t23417 = t2193 * t2193;
    let t23422 = -F::cast_from(0.77534644304710291488e-2_f64) * t95 * t104 * t23417 * t1975 - t22685 + t22687 - t22690 - t22694 - t22497 + t22562 + t22578 + t22581 - t22593 + t22697;
    (t23413, t23414, t23422)
}
