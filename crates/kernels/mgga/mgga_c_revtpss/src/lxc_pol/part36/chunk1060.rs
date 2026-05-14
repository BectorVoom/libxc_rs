//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1060/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1060<F: Float>(t7898: F, t7901: F, t4248: F, t7742: F, t28172: F, t7900: F, t2014: F, t2034: F, t22483: F, t30: F, t5966: F, t1963: F, t1544: F, t1583: F) -> (F, F, F, F, F, F, F, F, F) {
    let t29580 = 6.0 * t7898 * t7901;
    let t29582 = 4.0 * t4248 * t7742;
    let t29583 = t28172 * t7900;
    let t29585 = 6.0 * t2014 * t29583;
    let t29589 = t2034 * t22483;
    let t29590 = t2014 * t29589;
    let t29591 = t30 * t5966;
    let t29592 = t1963 * t29591;
    let t29598 = t1544 * t1583;
    (t29580, t29582, t29583, t29585, t29589, t29590, t29591, t29592, t29598)
}
