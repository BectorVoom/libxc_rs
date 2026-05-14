//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1375/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1375<F: Float>(t27017: F, t8392: F, t1882: F, t27265: F, t27289: F, t27199: F, t6636: F, t8232: F, t1017: F, t106803: F, t11593: F, t12715: F, t12956: F, t12968: F, t13140: F, t1643: F, t1901: F, t2221: F, t23478: F, t23555: F, t23571: F, t23581: F, t24078: F, t27011: F, t27020: F, t3052: F, t3439: F, t3455: F, t3478: F, t446: F, t47666: F, t50268: F, t574: F, t63304: F, t95521: F, t95813: F) -> (F,) {
    let t106928 = 4.0 / 9.0 * t8392 * t27017;
    let t106934 = 2.0 / 9.0 * t1882 * t27265;
    let t106940 = 4.0 / 9.0 * t1882 * t27289;
    let t106957 = 2.0 / 9.0 * t1882 * t27199;
    let t106958 = t8232 * t6636;
    let t106963 = -4.0 / 3.0 * t1901 * t50268 * t27011 - 4.0 / 3.0 * t1901 * t12968 * t95813 * t3478 - 4.0 / 3.0 * t1901 * t12968 * t23571 * t12956 + t106928 + 2.0 / 3.0 * t446 * t574 * t23478 * t3455 - t106934 + 2.0 / 27.0 * t1901 * t3439 * t27020 * t1643 - t106940 - 4.0 / 9.0 * t1901 * t63304 * t23555 + 4.0 / 9.0 * t11593 * t2221 * t23581 * t3052 - 4.0 / 3.0 * t1901 * t13140 * t95521 * t3478 - t446 * t574 * t24078 * t1017 / 3.0 + t106957 - 4.0 / 27.0 * t106958 - 4.0 / 27.0 * t47666 * t106803 * t12715;
    (t106963,)
}
