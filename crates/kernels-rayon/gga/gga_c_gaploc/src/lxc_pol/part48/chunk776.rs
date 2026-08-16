//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 776/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk776(t12448: f64, t2464: f64, t2487: f64, t4167: f64, t883: f64, t900: f64, t9086: f64, t20556: f64, t587: f64, t9438: f64, t1645: f64, t6949: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t40076 = t2487 * t2464 * t12448;
    let t40165 = t883 * t4167;
    let t40166 = t900 * t40165;
    let t40186 = t900 * t9086;
    let t40261 = t587 * t9438 * t20556;
    let t40342 = t1645 * t6949;
    (t40076, t40165, t40166, t40186, t40261, t40342)
}
