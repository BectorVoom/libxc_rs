//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1089/1468 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1089(t1033: f64, t5558: f64, t1381: f64, t2796: f64, t1023: f64, t5230: f64, t5508: f64, t1853: f64, t2926: f64, t24586: f64, t2610: f64, t1227: f64, t3091: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t27229 = t1033 * t5558;
    let t27232 = t2796 * t1381;
    let t27348 = t1023 * t5230;
    let t27403 = t1023 * t5508;
    let t27661 = t2926 * t1853;
    let t27728 = t2610 * t24586;
    let t27835 = t3091 * t1227;
    (t27229, t27232, t27348, t27403, t27661, t27728, t27835)
}
