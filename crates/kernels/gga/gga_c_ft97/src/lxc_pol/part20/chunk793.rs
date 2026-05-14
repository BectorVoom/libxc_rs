//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 793/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk793<F: Float>(t242: F, t24419: F, t1901: F, t24790: F, t24794: F, t24798: F, t24801: F, t24804: F, t24808: F, t24811: F, t24815: F, t24817: F, t24820: F, t24823: F, t24827: F, t24830: F, t24834: F, t24838: F, t24841: F, t24843: F, t446: F) -> (F, F) {
    let t24845 = t242 * t24419;
    let t24848 = 2.0 / 9.0 * t1901 * t24790 + 2.0 / 9.0 * t1901 * t24794 - 2.0 / 9.0 * t1901 * t24798 - t446 * t24801 / 3.0 + 2.0 / 3.0 * t446 * t24804 + 2.0 / 3.0 * t446 * t24808 - 2.0 / 3.0 * t446 * t24811 - t24815 + 2.0 / 27.0 * t1901 * t24817 - t446 * t24820 / 3.0 - 2.0 / 3.0 * t446 * t24823 - t446 * t24827 / 3.0 + 2.0 / 3.0 * t446 * t24830 - t446 * t24834 / 3.0 - 2.0 / 3.0 * t446 * t24838 + 2.0 / 9.0 * t24841 + 2.0 / 9.0 * t24843 + 4.0 / 3.0 * t446 * t24845;
    (t24845, t24848)
}
