//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 674/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk674<F: Float>(t1874: F, t2314: F, t4034: F, t1266: F, t1873: F) -> (F, F, F) {
    let t6522 = F::cast_from(2.0_f64) * t2314 * t1874;
    let t6524 = F::cast_from(2.0_f64) * t4034 * t1874;
    let t6525 = t1266 * t1873;
    (t6522, t6524, t6525)
}
