//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 680/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk680<F: Float>(t20902: F, t2179: F, t144: F, t3578: F, t4805: F, t167: F, t20655: F, t574: F, t1060: F, t4714: F, t13187: F, t17104: F, t17360: F, t17362: F, t17422: F, t1901: F, t20875: F, t20880: F, t20884: F, t20888: F, t20894: F, t20899: F, t446: F) -> (F, F, F, F, F, F, F) {
    let t20903 = t2179 * t20902;
    let t20904 = t144 * t20903;
    let t20908 = t3578 * t4805;
    let t20909 = t144 * t20908;
    let t20912 = t574 * t167 * t20655;
    let t20916 = t574 * t1060 * t4714;
    let t20919 = -2.0 / 3.0 * t1901 * t20875 - 2.0 / 9.0 * t17104 - 2.0 / 3.0 * t446 * t20880 + 2.0 / 3.0 * t446 * t20884 - 2.0 * t446 * t20888 + t17360 / 3.0 + 2.0 / 3.0 * t17362 + 2.0 * t446 * t20894 - 2.0 * t446 * t20899 + 2.0 * t446 * t20904 + 2.0 / 3.0 * t17422 - t446 * t20909 - t446 * t20912 / 3.0 - t446 * t20916 - 4.0 / 9.0 * t13187;
    (t20903, t20904, t20908, t20909, t20912, t20916, t20919)
}
