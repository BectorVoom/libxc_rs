//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1256/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1256<F: Float>(t18139: F, t193: F, t6008: F, t89: F, t30859: F, t668: F, t2354: F, t446: F, t505: F, t10157: F, t3821: F, t6852: F, t27882: F, t31052: F, t681: F, t31048: F) -> (F, F, F, F, F, F) {
    let t124127 = t89 * t193 * t6008 * t18139;
    let t124130 = t30859 * t668;
    let t124133 = t446 * t2354 * t124130 * t505;
    let t124137 = t446 * t10157 * t6852 * t3821;
    let t124141 = t89 * t193 * t27882 * t3821;
    let t124144 = t89 * t681 * t31052;
    let t124148 = t89 * t681 * t31048;
    (t124127, t124133, t124137, t124141, t124144, t124148)
}
