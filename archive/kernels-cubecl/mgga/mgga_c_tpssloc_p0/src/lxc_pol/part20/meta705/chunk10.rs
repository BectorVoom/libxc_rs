//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2688/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2688<F: Float>(t131: F, t205: F, t40024: F, t12012: F, t12156: F, t1315: F, t16084: F, t16101: F, t210: F, t214: F, t221: F, t3734: F, t46838: F, t5195: F, t5196: F, t53856: F, t54284: F, t54690: F, t54698: F, t54702: F, t54705: F, t54711: F, t54721: F, t54725: F) -> F {
    let t54728 = t205 * t40024 * t131;
    let t54736 = -F::cast_from(0.14999999999999999999e-1_f64) * t54690 + F::cast_from(0.49999999999999999998e-2_f64) * t5195 * t221 * t5196 * t12012 - F::cast_from(0.74999999999999999997e-2_f64) * t54698 + t54702 + F::cast_from(0.24999999999999999999e-2_f64) * t54705 - F::cast_from(0.16666666666666666666e-2_f64) * t1315 * t210 * t214 * t53856 - F::cast_from(0.69999999999999999996e-1_f64) * t54711 - F::cast_from(0.59999999999999999997e-1_f64) * t16101 * t221 * t16084 * t3734 + F::cast_from(0.29999999999999999998e-1_f64) * t54721 + F::cast_from(0.27777777777777777777e-3_f64) * t54725 + F::cast_from(0.99999999999999999995e-1_f64) * t54728 * t221 * t5196 * t12156 - F::cast_from(0.59999999999999999997e-1_f64) * t16101 * t46838 * t54284;
    t54736
}
