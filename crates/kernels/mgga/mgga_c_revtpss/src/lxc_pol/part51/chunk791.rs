//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 791/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk791<F: Float>(t28042: F, t508: F, t651: F, t118: F, t1519: F, t25805: F, t27145: F, t27152: F, t27156: F, t27830: F, t27834: F, t27835: F, t28022: F, t28025: F, t28030: F, t4254: F, t4257: F, t4293: F, t4297: F, t671: F, t6985: F, t7746: F) -> (F, F) {
    let t28043 = t508 * t28042;
    let t28045 = 2.0 * t651 * t28043;
    let t28046 = -t118 * t27830 - 2.0 * t1519 * t25805 - 2.0 * t1519 * t28025 - 2.0 * t27145 * t651 - 2.0 * t28030 * t671 - 2.0 * t4254 * t7746 - 2.0 * t4257 * t6985 - 2.0 * t4293 * t6985 - 2.0 * t4297 * t6985 + t27152 - t27156 + t27834 + t27835 + t28022 - t28045;
    (t28043, t28046)
}
