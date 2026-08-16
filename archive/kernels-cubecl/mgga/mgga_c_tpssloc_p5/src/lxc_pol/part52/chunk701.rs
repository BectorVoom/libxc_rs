//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 701/1400 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk701<F: Float>(t5: F, t6504: F, t67: F, t1864: F, t641: F, t71: F, t1863: F, t1860: F, t1865: F, t6486: F, t6490: F, t6492: F, t6495: F) -> (F, F, F, F, F) {
    let t7 = piecewise3::<F>(F::cast_from(0.0_f64) < t5, t5, -t5);
    let t8 = -t7 <= -F::cast_from(0.999999999999e0_f64);
    let t6505 = t6504 * t67;
    let t6506 = t6505 * t1864;
    let t6509 = t71 * t641;
    let t6510 = t1863 * t6509;
    let t6514 = piecewise3::<F>(t8, F::cast_from(0.0_f64), -t6486 * t1865 / F::cast_from(6.0_f64) + F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t6490 * t6492 + t6495 * t1865 / F::cast_from(3.0_f64) - t1860 * t6506 / F::cast_from(6.0_f64) - t1860 * t6510 / F::cast_from(6.0_f64));
    (t6505, t6506, t6509, t6510, t6514)
}
