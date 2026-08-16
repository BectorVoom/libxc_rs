//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 973/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk973(t40641: f64, t43072: f64, t44855: f64, t44857: f64, t44860: f64, t44861: f64, t44862: f64, t44863: f64, t44864: f64, t44865: f64, t739: f64, t1022: f64, t39048: f64, t787: f64) -> (f64, f64, f64) {
    let t50182 = t44855 - t44857 + 2.0_f64 * t43072 - 2.0_f64 * t40641 + t44860 + t44861 - t44862 + t44863 - t44864 - t44865;
    let t50183 = t739 * t50182;
    let t50194 = t787 * t39048 * t1022;
    (t50182, t50183, t50194)
}
