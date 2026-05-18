//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 946/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk946<F: Float>(t10879: F, t3305: F, t2172: F, t261: F, t3304: F, t2190: F, t3299: F, t2197: F, t7614: F, t2218: F, t503: F, t505: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t10880 = t10879 * t3305;
    let t10882 = t261 * t2172;
    let t10883 = t3304 * t10882;
    let t10885 = t261 * t2190;
    let t10886 = t3299 * t10885;
    let t10887 = F::new(0.23115257973478049502e0) * t10886;
    let t10888 = t261 * t2197;
    let t10889 = t7614 * t10888;
    let t10891 = t261 * t2218;
    let t10892 = t3304 * t10891;
    let t10893 = F::new(0.69345773920434148506e0) * t10892;
    let t10894 = t503 * t505;
    (t10880, t10882, t10883, t10885, t10886, t10887, t10888, t10889, t10891, t10892, t10893, t10894)
}
