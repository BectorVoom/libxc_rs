//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 586/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk586<F: Float>(t645: F, t4971: F, t4972: F, t1755: F, t4803: F, t1751: F, t1758: F, t340: F, t4962: F, t639: F, t642: F, t655: F, t1765: F, t1769: F, sigma2: F) -> (F, F, F, F, F) {
    let t646 = t645 < -F::new(0.66725e-1);
    let t4973 = t4971 * t4972;
    let t4977 = t1755 * t4803;
    let t4982 = piecewise3::<f64>(t646, F::new(0.0), F::new(10.0) / F::new(9.0) * t340 * t4962 * t642 - F::new(20.0) / F::new(27.0) * t340 * t1751 * t1758 + F::new(40.0) / F::new(81.0) * t340 * t639 * t4973 - F::new(10.0) / F::new(27.0) * t340 * t639 * t4977);
    let t4983 = t4982 * sigma2;
    let t4984 = t4983 * t655;
    let t4987 = t1765 * t1769;
    (t4973, t4977, t4983, t4984, t4987)
}
