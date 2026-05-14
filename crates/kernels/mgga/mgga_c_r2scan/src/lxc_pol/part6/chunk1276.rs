//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1276/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1276<F: Float>(t23939: F, t2625: F, t2858: F, t6599: F, t19611: F, t19614: F, t19620: F, t19624: F, t19628: F, t19646: F, t19649: F, t19720: F, t19728: F, t23937: F, t23938: F, t1543: F, t97: F) -> (F, F, F) {
    let t23940 = 0.7089e1 * t23939;
    let t23943 = 18.0 * t2858 * t6599 * t2625;
    let t23944 = t23937 + t23938 - t19720 - t23940 - t19611 - t19614 + t19620 - t19624 + t19628 + t19646 + t19649 + t19728 - t23943;
    let t23947 = t97 * t1543;
    (t23943, t23944, t23947)
}
