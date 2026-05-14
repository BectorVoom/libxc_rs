//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1465/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1465<F: Float>(t10576: F, t10591: F, t23312: F, t5026: F, t5028: F, t5030: F, t7109: F, t7127: F, t8650: F, t9783: F, t9786: F, t9790: F, t9918: F, t10581: F, t10584: F, t10587: F, t23258: F, t23263: F, t27461: F, t5035: F, t7157: F, t7159: F, t9824: F, t9827: F, t9833: F, t9925: F) -> (F, F) {
    let t35316 = 3.0 * t9783 + 180.0 * t7109 - t9918 + t10591 + t10576 + 18.0 * t9786 - 24.0 * t8650 + 18.0 * t9790 + t23312 + 0.10526802520742363173e2 * t7127 - t5026 - t5028 - t5030;
    let t35322 = t10581 + t23258 - t10584 + 3.0 * t9833 + 9.0 * t9824 + t10587 - t27461 - 0.52634012603711815863e1 * t7157 - 0.15584273195113317383e3 * t7159 + t5035 + 18.0 * t9827 + t23263 + t9925;
    (t35316, t35322)
}
