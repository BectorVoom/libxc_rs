//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 1010/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk1010(t33331: f64, t33332: f64, t43494: f64, t2660: f64, t33576: f64, t10930: f64, t10931: f64, t43508: f64, t10893: f64, t2628: f64, t10914: f64, t10915: f64) -> (f64, f64, f64, f64, f64) {
    let t44064 = 0.13803453343411469884e3_f64 * t33331 * t33332 * t43494;
    let t44065 = t33576 * t2660;
    let t44069 = 0.27606906686822939767e2_f64 * t10930 * t10931 * t43508;
    let t44070 = t10893 * t2628;
    let t44074 = 0.21450293971110256001e1_f64 * t10914 * t10915 * t43508;
    (t44064, t44065, t44069, t44070, t44074)
}
