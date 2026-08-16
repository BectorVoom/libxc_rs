//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1241/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1241<F: Float>(t112: F, t33578: F, t1873: F, t27188: F, t33234: F, t7042: F, t7467: F, t2039: F, t33211: F, t88: F, t7801: F, t8601: F) -> (F, F, F, F, F, F, F, F) {
    let t33579 = t33578 * t112;
    let t33583 = F::cast_from(2.0_f64) * t27188 * t1873;
    let t33585 = F::cast_from(2.0_f64) * t33234 * t1873;
    let t33587 = F::cast_from(2.0_f64) * t7042 * t7467;
    let t33595 = F::cast_from(2.0_f64) * t33211 * t2039;
    let t33596 = t88 * t7467;
    let t33598 = F::cast_from(2.0_f64) * t33596 * t2039;
    let t33600 = F::cast_from(2.0_f64) * t8601 * t7801;
    (t33579, t33583, t33585, t33587, t33595, t33596, t33598, t33600)
}
