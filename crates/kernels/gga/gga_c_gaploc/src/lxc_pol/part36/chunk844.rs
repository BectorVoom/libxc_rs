//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 844/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk844<F: Float>(t1445: F, t41869: F, t597: F, t10557: F, t9324: F, t41839: F, t6716: F, t6717: F, t41838: F, t475: F) -> (F, F, F, F) {
    let t41871 = t597 * t1445 * t41869;
    let t41874 = F::new(0.85801175884441024006e1) * t10557 * t9324;
    let t41876 = t6716 * t6717 * t41839;
    let t41878 = t41838 * t475;
    (t41871, t41874, t41876, t41878)
}
