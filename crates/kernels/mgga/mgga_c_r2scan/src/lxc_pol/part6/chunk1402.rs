//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1402/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1402<F: Float>(t1981: F, t5461: F, t898: F, t1859: F, t2816: F, t5377: F, t2810: F, t2813: F, t21649: F, t21659: F, t22114: F, t22116: F, t22125: F, t22126: F, t22128: F, t22130: F, t26520: F, t26522: F) -> (F,) {
    let t26525 = t898 * t1981 * t5461;
    let t26528 = t1859 * t2816 * t5377;
    let t26531 = t1859 * t2810 * t5377;
    let t26532 = 0.24012257405919999999e-1 * t26531;
    let t26534 = t1859 * t2813 * t5377;
    let t26535 = 0.24012257405919999999e-1 * t26534;
    let t26539 = -0.16936279733333333332e-2 * t26520 + 0.26345324029629629628e-2 * t26522 + 0.30762056574649219974e4 * t26525 - t21649 + t22114 - t22116 + 0.24012257405919999999e-1 * t26528 + t26532 + t26535 - t21659 + t22125 + 0.51947577317044391277e2 * t22126 + 0.30762056574649219974e4 * t22128 - 0.10526802520742363173e2 * t22130;
    (t26539,)
}
