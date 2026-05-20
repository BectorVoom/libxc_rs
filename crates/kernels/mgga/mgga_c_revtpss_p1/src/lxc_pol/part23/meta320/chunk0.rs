//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1609/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1609<F: Float>(t13632: F, t512: F, t9408: F, t9411: F, t1317: F, t5567: F, t2496: F, t5571: F, t5569: F, t9597: F, t123: F, t1856: F) -> (F, F, F, F, F, F, F, F) {
    let t13633 = t512 * t13632;
    let t13634 = F::new(32.0) * t9408;
    let t13635 = F::new(80.0) * t9411;
    let t13643 = F::new(8.0) * t1317 * t5567;
    let t13652 = t5571 * t2496;
    let t13654 = t1317 * t5569;
    let t13664 = F::new(12.0) * t9597;
    let t13665 = t1856 * t123;
    (t13633, t13634, t13635, t13643, t13652, t13654, t13664, t13665)
}
