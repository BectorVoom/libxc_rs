//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1342/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1342(t10624: f64, t1382: f64, t605: f64, t7329: f64, t8862: f64, t1960: f64, t8854: f64, t977: f64, t1959: f64, t3455: f64, t2497: f64, t2902: f64) -> (f64, f64, f64, f64, f64) {
    let t33986 = 4.0_f64 * t1382 * t10624 * t605;
    let t33988 = 4.0_f64 * t8862 * t7329;
    let t33991 = 2.0_f64 * t1960 * t8854 * t977;
    let t33992 = t3455 * t1959;
    let t33997 = 4.0_f64 * t1382 * t2902 * t2497;
    (t33986, t33988, t33991, t33992, t33997)
}
