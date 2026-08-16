//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2643/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2643<F: Float>(t28: F, t3673: F, t584: F, t1081: F, t3231: F, t16: F, t5181: F, t591: F, t11122: F, t12000: F, t12001: F, t1302: F, t16003: F, t16006: F, t1649: F, t2: F, t3711: F, t39877: F, t5178: F, zeta_threshold: F) -> (F, F, F, F, F) {
    let t29 = t28 <= zeta_threshold;
    let t53832 = t584 * t3673;
    let t53835 = t1081 * t3231;
    let t53841 = t16 * t1081;
    let t53844 = t584 * t3231;
    let t53852 = F::cast_from(16.0_f64) * t5181 * t591;
    let t53854 = piecewise3::<F>(t29, F::cast_from(0.0_f64), -F::cast_from(56.0_f64) / F::cast_from(81.0_f64) * t39877 * t1649 * t12001 - F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t12000 * t2 * t53832 + F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t16003 * t53835 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t3711 * t584 * t1081 - F::cast_from(4.0_f64) * t16006 * t53841 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t16006 * t53844 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t5178 * t11122 + F::cast_from(8.0_f64) * t1302 * t16 - t53852);
    (t53832, t53835, t53841, t53844, t53854)
}
