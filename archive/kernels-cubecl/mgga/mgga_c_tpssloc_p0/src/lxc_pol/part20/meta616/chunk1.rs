//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2225/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2225<F: Float>(t40: F, t10913: F, t12939: F, t4195: F, t12606: F, t12862: F, t12865: F, t1409: F, t2244: F, t2250: F, t2433: F, t3966: F, t40632: F, t4080: F, t45872: F, t607: F, t73: F, t9258: F, t9288: F, t9427: F, zeta_threshold: F) -> (F, F) {
    let t146 = t40 <= zeta_threshold;
    let t46152 = F::cast_from(72.0_f64) * t12939 * t4195 * t10913;
    let t46171 = piecewise3::<F>(t146, F::cast_from(0.0_f64), F::cast_from(40.0_f64) / F::cast_from(81.0_f64) * t40632 * t1409 * t9288 - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t9427 * t3966 * t2244 - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t12862 * t10913 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t2433 * t12606 * t607 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t12865 * t2250 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t4080 * t9258 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t73 * t45872);
    (t46152, t46171)
}
