//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1065/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1065(t20837: f64, t3491: f64, t91: f64, t446: f64, t569: f64, t85456: f64, t2205: f64, t85465: f64, t1969: f64, t86906: f64, t85474: f64, t1985: f64, t27: f64, t86681: f64, t89: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t86986 = t91 * t3491 * t20837;
    let t86989 = t446 * t569 * t85456;
    let t86992 = t446 * t2205 * t85465;
    let t86995 = t446 * t1969 * t86906;
    let t86998 = t446 * t569 * t85474;
    let t87002 = t89 * t27 * t1985 * t86681;
    (t86986, t86989, t86992, t86995, t86998, t87002)
}
