//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2269/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2269<F: Float>(t12568: F, t608: F, t2251: F, t3953: F, t1437: F, t2303: F, t72: F, t1865: F, t22523: F, t22554: F, t26055: F, t26063: F, t26067: F, t6490: F, t6506: F, t6510: F, t7432: F, t83750: F, t83760: F, t83775: F) -> F {
    let t90202 = t12568 * t608;
    let t90205 = t3953 * t2251;
    let t90227 = t72 * t2303 * t1437;
    let t90230 = F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t90202 * t1865 + t90205 * t1865 / F::cast_from(3.0_f64) + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t26055 * t6506 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t26055 * t6510 + F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t83775 * t7432 + F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t83750 * t7432 + F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t22554 * t26063 + F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t22554 * t26067 + F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t83760 * t7432 + F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t22523 * t26063 + F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t22523 * t26067 + F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t6490 * t90227;
    t90230
}
