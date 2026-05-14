//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1270/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1270<F: Float>(t28023: F, t3972: F, t1882: F, t31286: F, t31111: F, t8392: F, t31119: F, t10007: F, t110889: F, t110890: F, t11593: F, t123894: F, t123957: F, t124200: F, t14163: F, t18446: F, t18532: F, t1901: F, t242: F, t24526: F, t27468: F, t27767: F, t31129: F, t3880: F, t42575: F, t446: F, t5170: F, t53797: F, t6135: F, t68135: F, t98123: F) -> (F, F) {
    let t124361 = t28023 * t3972;
    let t124382 = t1882 * t31286;
    let t124397 = t8392 * t31111;
    let t124399 = t8392 * t31119;
    let t124401 = -t110889 - 8.0 / 81.0 * t110890 - 2.0 / 3.0 * t446 * t242 * t124361 - 4.0 / 9.0 * t1901 * t14163 * t123957 + 8.0 / 9.0 * t11593 * t14163 * t123894 - 2.0 / 9.0 * t1901 * t42575 * t31129 - 2.0 / 9.0 * t1901 * t10007 * t24526 * t5170 - 2.0 / 9.0 * t1901 * t10007 * t6135 * t18532 + 2.0 / 9.0 * t124382 - 4.0 / 9.0 * t1901 * t68135 * t27767 - 4.0 / 9.0 * t1901 * t14163 * t124200 - 2.0 / 9.0 * t1901 * t10007 * t27468 * t3880 + 4.0 / 9.0 * t53797 * t98123 * t18446 - 2.0 / 81.0 * t124397 + 4.0 / 9.0 * t124399;
    (t124361, t124401)
}
