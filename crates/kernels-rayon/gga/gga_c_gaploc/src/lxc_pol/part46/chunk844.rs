//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 844/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk844(t1445: f64, t41869: f64, t597: f64, t10557: f64, t9324: f64, t41839: f64, t6716: f64, t6717: f64, t41838: f64, t475: f64) -> (f64, f64, f64, f64) {
    let t41871 = t597 * t1445 * t41869;
    let t41874 = 0.85801175884441024006e1_f64 * t10557 * t9324;
    let t41876 = t6716 * t6717 * t41839;
    let t41878 = t41838 * t475;
    (t41871, t41874, t41876, t41878)
}
