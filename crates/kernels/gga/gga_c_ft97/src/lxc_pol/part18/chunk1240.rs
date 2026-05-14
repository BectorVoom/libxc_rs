//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1240/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1240<F: Float>(t1786: F, t6524: F, t22943: F, t463: F, t1882: F, t26255: F, t26468: F, t26472: F, t10964: F, t101573: F, t102376: F, t102417: F, t110: F, t11485: F, t11501: F, t11855: F, t11867: F, t12062: F, t1307: F, t1901: F, t1905: F, t3103: F, t446: F, t452: F, t5710: F, t5750: F, t83: F, t91629: F, t91771: F) -> (F, F) {
    let t102848 = t1786 * t6524;
    let t102862 = t463 * t22943;
    let t102878 = 4.0 / 9.0 * t1882 * t26255;
    let t102880 = 4.0 / 9.0 * t1882 * t26468;
    let t102882 = 4.0 / 9.0 * t1882 * t26472;
    let t102887 = t22943 * t10964;
    let t102891 = t446 * t452 * t5710 * t11501 / 3.0 + 8.0 / 27.0 * t91629 + 2.0 / 9.0 * t1901 * t102848 * t1905 - t446 * t83 * t102376 / 3.0 - t446 * t452 * t12062 * t1307 / 3.0 + 4.0 / 3.0 * t446 * t83 * t102417 - 4.0 / 9.0 * t1901 * t102862 * t11855 + 2.0 / 3.0 * t446 * t452 * t5710 * t11485 - 2.0 / 3.0 * t446 * t452 * t5750 * t3103 - 2.0 / 9.0 * t1901 * t91771 * t11867 - t102878 - t102880 - t102882 - t446 * t452 * t110 * t101573 / 3.0 + 2.0 / 3.0 * t446 * t83 * t102887;
    (t102887, t102891)
}
