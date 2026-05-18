//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1042/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1042<F: Float>(t2163: F, t7002: F, t651: F, t7003: F, t7586: F, t2322: F, t8749: F, t4254: F, t1936: F, t7683: F, t670: F, t8756: F) -> (F, F, F, F, F, F, F, F) {
    let t32855 = t2163 * t7002;
    let t32856 = t651 * t32855;
    let t32858 = t7586 * t7003;
    let t32862 = t2322 * t8749;
    let t32864 = t4254 * t8749;
    let t32866 = t7683 * t1936;
    let t32867 = t651 * t32866;
    let t32869 = t8756 * t670;
    (t32855, t32856, t32858, t32862, t32864, t32866, t32867, t32869)
}
