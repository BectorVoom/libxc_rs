//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1220/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1220<F: Float>(t33334: F, t533: F, t1390: F, t1983: F, t7802: F, t8526: F, t1799: F, t2018: F, t24432: F, t22574: F, t7685: F, t8644: F) -> (F, F, F, F, F, F, F, F) {
    let t33335 = t533 * t33334;
    let t33336 = t33335 * t1390;
    let t33337 = t1983 * t33336;
    let t33345 = F::cast_from(2.0_f64) * t8526 * t7802;
    let t33357 = t2018 * t1799;
    let t33358 = t24432 * t33357;
    let t33360 = F::cast_from(3.0_f64) * t22574 * t33358;
    let t33361 = t7685 * t8644;
    (t33335, t33336, t33337, t33345, t33357, t33358, t33360, t33361)
}
