//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2269/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2269<F: Float>(t1916: F, t26127: F, t26130: F, t1459: F, t28265: F, t26124: F, t28264: F, t4292: F, t572: F, t13514: F, t7330: F, t1518: F, t1936: F, t2371: F) -> (F, F, F, F, F, F, F) {
    let t101570 = F::new(6.0) * t1916 * t26127;
    let t101572 = F::new(3.0) * t1916 * t26130;
    let t101576 = F::new(12.0) * t1459 * t28265;
    let t101578 = F::new(12.0) * t1916 * t26124;
    let t101583 = F::new(12.0) * t572 * t28264 * t4292;
    let t101586 = F::new(6.0) * t572 * t7330 * t13514;
    let t101590 = F::new(6.0) * t572 * t2371 * t1936 * t1518;
    (t101570, t101572, t101576, t101578, t101583, t101586, t101590)
}
