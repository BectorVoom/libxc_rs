//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 609/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk609<F: Float>(t5: F, t2031: F, t7445: F, t1860: F, t2032: F, t7026: F, t7034: F, t7428: F, t7432: F, t7435: F, t112: F) -> (F, F, F) {
    let t7 = piecewise3::<F>(F::cast_from(0.0_f64) < t5, t5, -t5);
    let t8 = -t7 <= -F::cast_from(0.999999999999e0_f64);
    let t7782 = t2031 * t7445;
    let t7786 = piecewise3::<F>(t8, F::cast_from(0.0_f64), t7428 * t2032 / F::cast_from(3.0_f64) - F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t7026 * t7432 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t7435 * t2032 - t7034 + t1860 * t7782 / F::cast_from(3.0_f64));
    let t7787 = t7786 * t112;
    (t7782, t7786, t7787)
}
