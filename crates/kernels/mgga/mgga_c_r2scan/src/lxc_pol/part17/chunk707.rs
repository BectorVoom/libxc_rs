//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 707/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk707<F: Float>(t1399: F, t1660: F, t1665: F, t2005: F, t206: F, t1923: F, t2008: F, t1966: F, t689: F, t1937: F, t681: F, t686: F) -> (F, F, F, F, F, F) {
    let t5612 = F::new(0.14246666666666666666e0) * t1399 * t1660;
    let t5614 = F::new(0.11455730062901982479e1) * t1399 * t1665;
    let t5627 = t2005 * t206;
    let t5628 = t2008 * t1923;
    let t5629 = t5627 * t5628;
    let t5632 = t689 * t1966;
    let t5633 = t1937 * t5632;
    let t5636 = t686 * t681;
    (t5612, t5614, t5629, t5632, t5633, t5636)
}
