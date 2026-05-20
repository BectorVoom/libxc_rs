//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1651/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1651<F: Float>(t14535: F, t231: F, t2783: F, t2782: F, t10867: F, t225: F, t213: F) -> (F, F, F, F) {
    let t14537 = t2783 * t14535 * t231;
    let t14539 = F::cast_from(0.10975748638225852664e-1_f64) * t2782 * t14537;
    let t14545 = t225 * t10867;
    let t14546 = t213 * t14545;
    (t14537, t14539, t14545, t14546)
}
