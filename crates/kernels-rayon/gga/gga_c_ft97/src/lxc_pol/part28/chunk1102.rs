//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1102/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1102(t26706: f64, t32152: f64, t26722: f64, t26715: f64, t32233: f64, t138738: f64, t3392: f64, t26696: f64, t1008: f64, t7189: f64, t137007: f64, t554: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t147231 = t32152 * t26706;
    let t147234 = t32152 * t26722;
    let t147238 = t32233 * t26715;
    let t147243 = t3392 * t138738;
    let t147248 = t32152 * t26696;
    let t147251 = t7189 * t1008;
    let t147253 = t137007 * t147251 * t554;
    (t147231, t147234, t147238, t147243, t147248, t147253)
}
