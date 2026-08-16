//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1037/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1037(t1756: f64, t2084: f64, t2145: f64, t27: f64, t1818: f64, t236: f64, t3351: f64, t40168: f64, t498: f64, t10018: f64, t7255: f64, t1910: f64, t495: f64, t7230: f64, t7231: f64) -> (f64, f64, f64, f64) {
    let t47616 = t2145 * t27 * t2084 * t1756;
    let t47621 = t3351 * t40168 * t236 * t1818 * t498;
    let t47623 = t7255 * t10018;
    let t47629 = t7230 * t7231 * t236 * t1910 * t495;
    (t47616, t47621, t47623, t47629)
}
