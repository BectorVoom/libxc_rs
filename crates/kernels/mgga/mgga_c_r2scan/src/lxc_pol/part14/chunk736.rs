//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 736/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk736<F: Float>(t1784: F, t584: F, t591: F, t1789: F, t406: F, t410: F, t1748: F, t1751: F, t1398: F, t745: F, t735: F, t1668: F) -> (F, F, F, F, F, F) {
    let t5930 = t584 * t1784 * t591;
    let t5932 = t406 * t1789;
    let t5934 = t410 * t1789;
    let t5936 = t1751 * t1748;
    let t5938 = t1398 * t745;
    let t5940 = F::cast_from(0.21687162600603479684e-1_f64) * t735 * t5938;
    let t5942 = t1668 * t591;
    (t5930, t5932, t5934, t5936, t5940, t5942)
}
