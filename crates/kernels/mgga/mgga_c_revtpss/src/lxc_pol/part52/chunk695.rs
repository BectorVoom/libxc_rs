//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 695/1144 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk695<F: Float>(t5: F, t1916: F, t2042: F, t1518: F, t7330: F, t572: F, t117: F, t7741: F, t1918: F, t2040: F, t573: F, t7944: F, t2047: F, t7719: F, t1923: F, t2048: F, t7343: F, t7351: F, t7702: F, t7706: F, t7709: F) -> (F, F, F, F, F) {
    let t7 = piecewise3(0.0 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0;
    let t7949 = 3.0 * t1916 * t2042;
    let t7950 = t7330 * t1518;
    let t7952 = 6.0 * t572 * t7950;
    let t7953 = t117 * t7741;
    let t7955 = 3.0 * t572 * t7953;
    let t7956 = 3.0 * t1918 * t2040 + t573 * t7944 + t7949 + t7952 + t7955;
    let t7964 = t2047 * t7719;
    let t7968 = piecewise3(t8, 0.0, t7702 * t2048 / 3.0 - 5.0 / 3.0 * t7343 * t7706 - 2.0 / 3.0 * t7709 * t2048 - t7351 + t1923 * t7964 / 3.0);
    (t7950, t7953, t7956, t7964, t7968)
}
