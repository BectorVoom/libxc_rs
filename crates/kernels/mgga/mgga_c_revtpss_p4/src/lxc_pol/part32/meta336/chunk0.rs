//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1260/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1260<F: Float>(t9395: F, t2626: F, t5571: F, t1856: F, t2608: F, t512: F, t9408: F, t9411: F, t1317: F, t5567: F, t4147: F, t5778: F) -> (F, F, F, F, F, F, F) {
    let t13623 = F::new(4.0) * t9395;
    let t13630 = t5571 * t2626;
    let t13632 = t1856 * t2608;
    let t13633 = t512 * t13632;
    let t13634 = F::new(32.0) * t9408;
    let t13635 = F::new(80.0) * t9411;
    let t13643 = F::new(8.0) * t1317 * t5567;
    let t13648 = t5778 * t4147;
    (t13623, t13630, t13633, t13634, t13635, t13643, t13648)
}
