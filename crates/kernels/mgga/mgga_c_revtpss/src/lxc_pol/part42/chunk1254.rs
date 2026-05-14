//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1254/1363 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1254<F: Float>(t12809: F, t12853: F, t17290: F, t17386: F, t17417: F, t17425: F, t17605: F, t17729: F, t17753: F, t1791: F, t21030: F, t21037: F, t21042: F, t21046: F, t21049: F, t21053: F, t3718: F, t5343: F, t5402: F) -> (F,) {
    let t21057 = 0.42874018118069736972e-3 * t12809 * t21030 + 0.15244095330869239812e-2 * t17605 * t5402 - t17386 + 0.57165357490759649296e-3 * t17729 * t21037 - 0.21437009059034868486e-3 * t3718 * t21042 + 0.21437009059034868486e-3 * t17753 * t21046 + 0.85748036236139473944e-3 * t21049 * t5343 + 0.6351706387862183255e-4 * t17417 + t12853 + t17425 - 0.28582678745379824648e-3 * t21053 - 0.42874018118069736972e-3 * t17290 * t1791;
    (t21057,)
}
