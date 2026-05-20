//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 951/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk951<F: Float>(t7003: F, t7586: F, t2322: F, t8749: F, t4254: F, t1936: F, t7683: F, t651: F, t2042: F, t7696: F, t2170: F, t7331: F) -> (F, F, F, F, F, F, F) {
    let t32858 = t7586 * t7003;
    let t32862 = t2322 * t8749;
    let t32864 = t4254 * t8749;
    let t32866 = t7683 * t1936;
    let t32867 = t651 * t32866;
    let t32901 = t7696 * t2042;
    let t32903 = t2170 * t7331;
    (t32858, t32862, t32864, t32866, t32867, t32901, t32903)
}
