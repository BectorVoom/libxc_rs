//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 625/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk625<F: Float>(t1882: F, t2182: F, t2187: F, t2202: F, t161: F, t7943: F, t89: F, t1853: F, t979: F, t8418: F, t3255: F, t492: F, t1852: F, t1820: F, t3219: F, t8466: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t9449 = t1882 * t2182;
    let t9451 = t1882 * t2187;
    let t9453 = t1882 * t2202;
    let t9457 = 28.0 / 81.0 * t89 * t7943 * t161;
    let t10951 = t979 * t1853;
    let t10952 = t8418 * t10951;
    let t10961 = t3255 * t492;
    let t10962 = t1852 * t10961;
    let t10964 = t979 * t1820;
    let t10965 = t1852 * t10964;
    let t10967 = t8466 * t3219;
    (t9449, t9451, t9453, t9457, t10951, t10952, t10961, t10962, t10964, t10965, t10967)
}
