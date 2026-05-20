//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1158/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1158<F: Float>(t34428: F, t4254: F, t651: F, t7683: F, t7741: F, t28189: F, t8764: F, t32855: F, t7732: F, t2322: F, t34382: F, t1936: F, t29337: F) -> (F, F, F, F, F, F, F) {
    let t129316 = t4254 * t34428;
    let t129319 = t651 * t7683 * t7741;
    let t129322 = t8764 * t28189;
    let t129326 = t7732 * t32855;
    let t129328 = t2322 * t34382;
    let t129332 = t4254 * t34382;
    let t129335 = t651 * t29337 * t1936;
    (t129316, t129319, t129322, t129326, t129328, t129332, t129335)
}
