//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1057/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1057<F: Float>(t31914: F, t31929: F, t295: F, t312: F, t1501: F, t5309: F, t10697: F, t296: F, t1091: F, t7131: F, t835: F, t15128: F, t7114: F, t1901: F, t193: F, t29385: F, t29387: F, t29392: F, t29405: F, t31854: F, t31859: F, t31864: F, t31869: F, t31873: F, t31877: F, t31881: F, t31885: F, t31891: F, t31895: F, t446: F, t89: F) -> (F, F, F, F, F, F, F, F, F) {
    let t31930 = t31914 + t31929;
    let t31932 = t295 * t31930 * t312;
    let t31936 = t1501 * t5309;
    let t31937 = t10697 * t31936;
    let t31938 = t296 * t31937;
    let t31942 = t835 * t7131 * t1091;
    let t31945 = t15128 * t7114;
    let t31946 = t296 * t31945;
    let t31950 = t446 * t31854 / 3.0 - 2.0 / 3.0 * t446 * t31859 + t446 * t31864 / 3.0 + 2.0 / 3.0 * t446 * t31869 + 2.0 / 3.0 * t446 * t31873 + 2.0 / 3.0 * t446 * t31877 + 4.0 / 3.0 * t446 * t31881 + 4.0 / 3.0 * t446 * t31885 + 2.0 / 9.0 * t29385 + 2.0 / 9.0 * t29387 + 2.0 / 27.0 * t1901 * t31891 - 4.0 / 3.0 * t1901 * t31895 + 2.0 / 9.0 * t29392 + t89 * t193 * t31932 / 3.0 - 2.0 * t446 * t31938 - 2.0 / 9.0 * t446 * t31942 + 4.0 / 3.0 * t446 * t31946 - 2.0 / 9.0 * t29405;
    (t31930, t31932, t31936, t31937, t31938, t31942, t31945, t31946, t31950)
}
