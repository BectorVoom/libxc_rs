//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1825/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1825<F: Float>(t27137: F, t651: F, t7235: F, t7935: F, t1353: F, t1907: F, t8717: F, t25082: F, t1962: F, t198: F, t205: F) -> (F, F, F, F, F, F) {
    let t27139 = F::new(2.0) * t651 * t27137;
    let t27152 = t7235 * t7935;
    let t27153 = t1907 * t1353;
    let t27154 = t8717 * t27153;
    let t27156 = F::new(3.0) * t25082 * t27154;
    let t27158 = t198 * t205 * t1962;
    (t27139, t27152, t27153, t27154, t27156, t27158)
}
