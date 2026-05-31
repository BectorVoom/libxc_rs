//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2326/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2326<F: Float>(t1501: F, t1518: F, t10208: F, t69: F, t26: F, t65: F, t1651: F, t385: F, t1774: F, t494: F, t9163: F, t99: F) -> (F, F, F, F, F, F) {
    let t30138 = t1501 * t1518;
    let t31035 = t69 * t10208;
    let t33127 = F::cast_from(1.0_f64) / t65 / t26;
    let t33754 = t385 * t1651;
    let t34934 = t494 * t1774;
    let t36227 = t99 * t9163;
    (t30138, t31035, t33127, t33754, t34934, t36227)
}
