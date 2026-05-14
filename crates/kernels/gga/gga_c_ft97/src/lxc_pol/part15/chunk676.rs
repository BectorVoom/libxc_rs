//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 676/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk676<F: Float>(t20655: F, t24: F, t586: F, t20660: F, t9236: F, t1985: F, t3518: F, t4714: F, t20348: F, t9224: F, t12809: F, t12852: F, t17272: F, t17274: F, t17276: F, t17310: F, t20810: F, t20813: F, t462: F, t92: F) -> (F, F, F, F, F) {
    let t20818 = t24 * t586 * t20655;
    let t20823 = t24 * t9236 * t20660;
    let t20827 = t1985 * t3518 * t4714;
    let t20830 = t9224 * t20348;
    let t20836 = -2.0 * t462 * t20810 + 2.0 * t462 * t20813 - 4.0 / 9.0 * t12852 - t92 * t20818 - 2.0 / 3.0 * t17310 - 4.0 / 3.0 * t12809 - 6.0 * t92 * t20823 + 6.0 * t462 * t20827 - 10.0 / 27.0 * t462 * t20830 - 2.0 / 3.0 * t17272 + t17274 / 3.0 + 2.0 / 9.0 * t17276;
    (t20818, t20823, t20827, t20830, t20836)
}
