//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1296/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1296<F: Float>(t159: F, t2698: F, t1544: F, t1583: F, t1868: F, t1907: F, t1501: F, t1518: F, t26: F, t65: F, t9163: F, t99: F) -> (F, F, F, F, F, F) {
    let t25273 = t2698 * t159;
    let t29598 = t1544 * t1583;
    let t30122 = t1868 * t1907;
    let t30138 = t1501 * t1518;
    let t33127 = F::new(1.0) / t65 / t26;
    let t36227 = t99 * t9163;
    (t25273, t29598, t30122, t30138, t33127, t36227)
}
