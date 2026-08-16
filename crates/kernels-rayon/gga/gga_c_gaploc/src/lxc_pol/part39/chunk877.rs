//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 877/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk877(t12526: f64, t2487: f64, t6985: f64, t2365: f64, t30209: f64, t7025: f64, t2610: f64, t28023: f64, t1843: f64, t9647: f64, t2563: f64, t9756: f64) -> (f64, f64, f64, f64, f64) {
    let t40567 = t2487 * t6985 * t12526;
    let t40570 = t7025 * t2365 * t30209;
    let t40586 = t2610 * t28023;
    let t40588 = t9647 * t1843 * t40586;
    let t40591 = t9647 * t9756 * t2563;
    (t40567, t40570, t40586, t40588, t40591)
}
