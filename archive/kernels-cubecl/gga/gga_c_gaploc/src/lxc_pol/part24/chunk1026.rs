//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1026/1439 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1026<F: Float>(t10668: F, t531: F, t10667: F, t808: F, t568: F, t836: F, t1628: F, t3507: F, t10019: F, t2617: F, t3005: F, t7810: F) -> (F, F, F, F, F, F, F, F, F) {
    let t11090 = t531 * t10668;
    let t11095 = t808 * t10667;
    let t11096 = t568 * t11095;
    let t11101 = t836 * t10667;
    let t11102 = t568 * t11101;
    let t11105 = t1628 * t3507;
    let t11108 = F::cast_from(0.15976219147466979032e-1_f64) * t10019;
    let t11109 = t3005 * t2617;
    let t11110 = t7810 * t11109;
    (t11090, t11095, t11096, t11101, t11102, t11105, t11108, t11109, t11110)
}
