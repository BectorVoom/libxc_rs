//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 773/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk773(t40165: f64, t900: f64, t9086: f64, t20556: f64, t587: f64, t9438: f64, t1645: f64, t6949: f64, t20700: f64, t6710: f64, t20551: f64, t6914: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t40166 = t900 * t40165;
    let t40186 = t900 * t9086;
    let t40261 = t587 * t9438 * t20556;
    let t40342 = t1645 * t6949;
    let t40372 = t6710 * t9438 * t20700;
    let t40377 = t6914 * t9438 * t20551;
    (t40166, t40186, t40261, t40342, t40372, t40377)
}
