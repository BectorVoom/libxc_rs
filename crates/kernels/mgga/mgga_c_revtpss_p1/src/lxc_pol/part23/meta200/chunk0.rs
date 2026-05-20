//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1192/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1192<F: Float>(t1353: F, t5651: F, t1394: F, t5591: F, t1392: F, t1395: F, t1877: F, t1879: F, t539: F, t541: F, t5644: F, t5650: F) -> (F, F, F) {
    let t5652 = t5651 * t1353;
    let t5655 = t1394 * t5591;
    let t5658 = F::new(3.0) * t1392 * t1879 + F::new(3.0) * t1395 * t1877 + F::new(3.0) * t539 * t5655 - t541 * t5644 - F::new(12.0) * t5650 * t5652;
    (t5652, t5655, t5658)
}
