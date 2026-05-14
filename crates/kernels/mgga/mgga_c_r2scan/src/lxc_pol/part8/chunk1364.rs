//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1364/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1364<F: Float>(t10111: F, t1592: F, t1632: F, t551: F, t24928: F, t24948: F, t24963: F, t24967: F, t24971: F, t25206: F, t29487: F, t29498: F, t29502: F, t29515: F, t29524: F, t29533: F, t29544: F, t3077: F) -> (F,) {
    let t33391 = t1592 * t551 * t1632 * t10111;
    let t33394 = 0.34672886960217074253e0 * t29487 + 0.19756347548806534796e0 * t29498 - 0.98781737744032673978e-1 * t29502 + t24928 - 0.6402520038965080721e0 * t29515 - 0.13869154784086829701e1 * t29524 + 0.26004665220162805689e0 * t25206 * t3077 - 0.6402520038965080721e0 * t29533 - 0.48787202696913915094e-3 * t24948 - 0.10401866088065122276e1 * t33391 - 0.83214928704520978206e1 * t29544 - t24963 + t24967 + t24971;
    (t33394,)
}
