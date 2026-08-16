//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2688/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2688(t131: f64, t205: f64, t40024: f64, t12012: f64, t12156: f64, t1315: f64, t16084: f64, t16101: f64, t210: f64, t214: f64, t221: f64, t3734: f64, t46838: f64, t5195: f64, t5196: f64, t53856: f64, t54284: f64, t54690: f64, t54698: f64, t54702: f64, t54705: f64, t54711: f64, t54721: f64, t54725: f64) -> f64 {
    let t54728 = t205 * t40024 * t131;
    let t54736 = -0.14999999999999999999e-1_f64 * t54690 + 0.49999999999999999998e-2_f64 * t5195 * t221 * t5196 * t12012 - 0.74999999999999999997e-2_f64 * t54698 + t54702 + 0.24999999999999999999e-2_f64 * t54705 - 0.16666666666666666666e-2_f64 * t1315 * t210 * t214 * t53856 - 0.69999999999999999996e-1_f64 * t54711 - 0.59999999999999999997e-1_f64 * t16101 * t221 * t16084 * t3734 + 0.29999999999999999998e-1_f64 * t54721 + 0.27777777777777777777e-3_f64 * t54725 + 0.99999999999999999995e-1_f64 * t54728 * t221 * t5196 * t12156 - 0.59999999999999999997e-1_f64 * t16101 * t46838 * t54284;
    t54736
}
