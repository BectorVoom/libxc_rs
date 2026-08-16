//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 733/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk733(t6895: f64, t912: f64, t587: f64, t2478: f64, t589: f64, t2476: f64, t549: f64, t6536: f64, t161: f64, t4774: f64) -> (f64, f64, f64, f64) {
    let t6896 = t912 * t6895;
    let t6897 = t587 * t6896;
    let t6899 = t589 * t2478;
    let t6900 = t2476 * t6899;
    let t6904 = t549 * t6536;
    let t6907 = t161 * t4774;
    (t6897, t6900, t6904, t6907)
}
