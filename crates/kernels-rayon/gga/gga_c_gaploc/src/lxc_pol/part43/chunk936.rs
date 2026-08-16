//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 936/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk936(t10930: f64, t10931: f64, t43494: f64, t33331: f64, t33332: f64, t43508: f64, t10914: f64, t10915: f64, t10811: f64, t9961: f64, t1022: f64, t9636: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t44060 = 0.38649669361552115674e3_f64 * t10930 * t10931 * t43494;
    let t44064 = 0.13803453343411469884e3_f64 * t33331 * t33332 * t43494;
    let t44069 = 0.27606906686822939767e2_f64 * t10930 * t10931 * t43508;
    let t44074 = 0.21450293971110256001e1_f64 * t10914 * t10915 * t43508;
    let t44079 = 0.85801175884441024006e1_f64 * t10811 * t9961;
    let t44080 = t9636 * t1022;
    (t44060, t44064, t44069, t44074, t44079, t44080)
}
