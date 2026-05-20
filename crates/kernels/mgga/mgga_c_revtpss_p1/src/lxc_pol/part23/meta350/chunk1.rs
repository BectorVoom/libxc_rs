//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1657/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1657<F: Float>(t14616: F, t757: F, t73: F, t830: F, t1544: F, t2475: F, t4343: F, t853: F, t124: F, t1558: F) -> (F, F, F, F, F) {
    let t14618 = F::cast_from(0.36622894612013090108e-3_f64) * t14616 * t757;
    let t14643 = t830 * t73;
    let t14648 = t2475 * t1544;
    let t14652 = t853 * t4343;
    let t14671 = t124 * t1558;
    (t14618, t14643, t14648, t14652, t14671)
}
