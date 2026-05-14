//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 1078/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk1078<F: Float>(t128: F, t203: F, t11417: F, t457: F, t5741: F, t632: F, t1781: F, t3684: F, t11357: F, t27063: F, t34607: F, t5967: F, t1743: F, t20501: F, t33429: F, t11381: F, t9061: F) -> (F, F, F, F, F, F, F) {
    let t34863 = t203 * t128;
    let t34866 = t632 * t11417 * t5741 * t34863 * t457;
    let t34868 = t3684 * t1781;
    let t34870 = t11357 * t27063;
    let t34873 = t34607 * t5967;
    let t34876 = t1743 * t33429 * t20501;
    let t34878 = t9061 * t11381;
    (t34863, t34866, t34868, t34870, t34873, t34876, t34878)
}
