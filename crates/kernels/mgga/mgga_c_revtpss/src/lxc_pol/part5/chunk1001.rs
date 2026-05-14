//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1001/1286 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk1001<F: Float>(t14605: F, t2482: F, t2801: F, t1531: F, t37: F, t4392: F, t72: F, t757: F, t1544: F, t2475: F, t124: F, t1558: F, t10779: F, t2749: F, t10777: F, t125: F, t4423: F) -> (F, F, F, F, F, F, F) {
    let t14606 = t2482 * t14605;
    let t14608 = 0.19514881078765566038e-1 * t14606 * t2801;
    let t14613 = t37 * t1531;
    let t14616 = t4392 * t72;
    let t14618 = 0.36622894612013090108e-3 * t14616 * t757;
    let t14648 = t2475 * t1544;
    let t14671 = t124 * t1558;
    let t14673 = t10779 * t14671 * t2749;
    let t14675 = 0.10164000561857065645e-3 * t10777 * t14673;
    let t14676 = t125 * t4423;
    (t14608, t14613, t14618, t14648, t14671, t14675, t14676)
}
