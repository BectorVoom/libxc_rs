//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1916/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1916<F: Float>(t11094: F, t1637: F, t17202: F, t193: F, t21093: F, t21097: F, t21099: F, t21103: F, t21105: F, t21107: F, t21365: F, t21367: F, t21369: F, t21372: F, t21375: F, t21376: F, t336: F, t4700: F) -> F {
    let t21381 = F::cast_from(2.0_f64) * t11094 * t193 * t21376 * t336 - F::cast_from(3.0_f64) * t1637 * t17202 * t4700 - t21093 + t21097 - t21099 - t21103 - t21105 - t21107 + t21365 + t21367 + t21369 - t21372 + t21375;
    t21381
}
