//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 725/1196 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk725<F: Float>(t3: F, t7939: F, t1916: F, t2042: F, t1518: F, t7330: F, t572: F, t117: F, t7741: F, t1918: F, t2040: F, t573: F, t3140: F, t3268: F, t1078: F, t1035: F) -> (F, F, F, F, F, F, F) {
    let t7940 = t3 * t7939;
    let t7944 = param_d * t7939;
    let t7949 = 3.0 * t1916 * t2042;
    let t7950 = t7330 * t1518;
    let t7952 = 6.0 * t572 * t7950;
    let t7953 = t117 * t7741;
    let t7955 = 3.0 * t572 * t7953;
    let t7956 = 3.0 * t1918 * t2040 + t573 * t7944 + t7949 + t7952 + t7955;
    let t8515 = t3140 * t3268;
    let t8520 = t3140 * t1078;
    let t8521 = t8520 * t1035;
    (t7940, t7944, t7950, t7953, t7956, t8515, t8521)
}
