//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1337/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1337<F: Float>(t2572: F, t264: F, t1579: F, t20344: F, t20348: F, t20357: F, t20361: F, t20363: F, t25179: F, t25182: F, t25185: F, t25189: F, t25194: F, t25198: F, t25206: F, t25210: F, t25215: F, t6134: F, t6366: F, t7339: F) -> (F, F) {
    let t25216 = t264 * t2572;
    let t25222 = 0.41917145582815912122e0 * t25179 + t25182 - 0.49390868872016336991e-1 * t25185 + t25189 - 0.16463622957338778996e-1 * t20344 + 0.82318114786693894983e-2 * t25194 - 0.2600466522016280569e0 * t25198 * t6366 - 0.69345773920434148506e0 * t20348 + 0.41607464352260489103e1 * t20357 - 0.41607464352260489103e1 * t20361 + 0.26004665220162805689e0 * t25206 * t1579 - 0.73613752582167450608e0 * t25210 + 0.15602799132097683414e1 * t25215 * t25216 * t6134 * t7339 - 0.49390868872016336991e-1 * t20363;
    (t25216, t25222)
}
