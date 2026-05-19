//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 523/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk523<F: Float>(t11: F, t1643: F, t1645: F, t2736: F, t2804: F, t2819: F, t2828: F, t5: F, param_eta: F) -> F {
    let t2832 = t1643 - F::new(5.0) / F::new(3.0) * t1645 - F::new(5.0) / F::new(3.0) * t2736 + F::new(5.0) * t5 * t11 * t2804 - F::new(45.0) * param_eta * (t2819 + t2828);
    t2832
}
