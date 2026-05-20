//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1824/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1824<F: Float>(t1937: F, t27123: F, t4292: F, t94: F, t6993: F, t7732: F, t7003: F, t2322: F, t7735: F, t4254: F, t1936: F, t5517: F) -> (F, F, F, F, F, F, F, F) {
    let t27125 = F::new(2.0) * t27123 * t1937;
    let t27126 = t94 * t4292;
    let t27128 = F::new(2.0) * t27126 * t1937;
    let t27130 = F::new(2.0) * t7732 * t6993;
    let t27132 = F::new(2.0) * t7732 * t7003;
    let t27134 = F::new(2.0) * t2322 * t7735;
    let t27136 = F::new(2.0) * t4254 * t7735;
    let t27137 = t5517 * t1936;
    (t27125, t27126, t27128, t27130, t27132, t27134, t27136, t27137)
}
