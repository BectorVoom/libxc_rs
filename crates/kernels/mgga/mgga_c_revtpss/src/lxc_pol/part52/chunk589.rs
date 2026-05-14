//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 589/1144 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk589<F: Float>(t5: F, t6972: F, t72: F, t1927: F, t640: F, t76: F, t1926: F, t1923: F, t1928: F, t6954: F, t6958: F, t6960: F, t6963: F, t117: F) -> (F, F, F, F, F, F) {
    let t7 = piecewise3(0.0 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0;
    let t6973 = t6972 * t72;
    let t6974 = t6973 * t1927;
    let t6977 = t76 * t640;
    let t6978 = t1926 * t6977;
    let t6982 = piecewise3(t8, 0.0, -t6954 * t1928 / 6.0 + 5.0 / 6.0 * t6958 * t6960 + t6963 * t1928 / 3.0 - t1923 * t6974 / 6.0 - t1923 * t6978 / 6.0);
    let t6983 = t6982 * t117;
    (t6973, t6974, t6977, t6978, t6982, t6983)
}
