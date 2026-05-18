//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1299/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1299<F: Float>(t237: F, t31151: F, t31191: F, t31309: F, t31345: F, t31391: F, t31437: F, t31517: F, t31587: F, t1217: F, t27501: F, t11353: F, t2328: F) -> (F, F, F) {
    let t31591 = t237 * (t31151 + t31191 + t31309 + t31345 + t31391 + t31437 + t31517 + t31587);
    let t31593 = F::new(0.17544670867903938621e1) * t27501 * t1217;
    let t31595 = F::new(0.5848223622634646207e0) * t2328 * t11353;
    (t31591, t31593, t31595)
}
