//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 740/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk740<F: Float>(t27153: F, t8717: F, t25082: F, t1962: F, t198: F, t205: F, t30: F, t892: F, t4433: F, t18875: F, t25207: F, t1544: F, t605: F, t4343: F, t1949: F, t4533: F) -> (F, F, F, F, F, F, F, F) {
    let t27154 = t8717 * t27153;
    let t27156 = 3.0 * t25082 * t27154;
    let t27158 = t198 * t205 * t1962;
    let t27159 = t892 * t30;
    let t27160 = t27159 * t4433;
    let t27166 = t25207 * t18875;
    let t27169 = t605 * t1544;
    let t27173 = t30 * t4343;
    let t27182 = t1949 * t4533;
    (t27154, t27156, t27158, t27160, t27166, t27169, t27173, t27182)
}
