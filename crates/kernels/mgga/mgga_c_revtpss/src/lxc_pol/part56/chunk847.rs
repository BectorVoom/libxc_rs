//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 847/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk847<F: Float>(t1936: F, t7683: F, t651: F, t2042: F, t7696: F, t2170: F, t7331: F, t7334: F, t670: F, t8964: F, t116: F, t8916: F) -> (F, F, F, F, F, F, F) {
    let t32866 = t7683 * t1936;
    let t32867 = t651 * t32866;
    let t32901 = t7696 * t2042;
    let t32903 = t2170 * t7331;
    let t32905 = t2170 * t7334;
    let t33343 = t8964 * t670;
    let t33346 = t8916 * t116;
    (t32866, t32867, t32901, t32903, t32905, t33343, t33346)
}
