//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1230/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1230<F: Float>(t26517: F, t5403: F, t7784: F, t2799: F, t5465: F, t1981: F, t5461: F, t898: F, t1859: F, t2816: F, t5377: F, t2810: F, t2813: F, t406: F, t7794: F, t1871: F, t2782: F, t584: F) -> (F, F, F, F, F, F, F, F, F) {
    let t26518 = 0.33872559466666666666e-2 * t26517;
    let t26520 = t7784 * t5403;
    let t26522 = t2799 * t5465;
    let t26525 = t898 * t1981 * t5461;
    let t26528 = t1859 * t2816 * t5377;
    let t26531 = t1859 * t2810 * t5377;
    let t26532 = 0.24012257405919999999e-1 * t26531;
    let t26534 = t1859 * t2813 * t5377;
    let t26535 = 0.24012257405919999999e-1 * t26534;
    let t26555 = t406 * t7794;
    let t26556 = 12.0 * t26555;
    let t26560 = t584 * t2782 * t1871;
    (t26518, t26520, t26522, t26525, t26528, t26532, t26535, t26556, t26560)
}
