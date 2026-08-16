//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1133/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1133<F: Float>(t1086: F, t3046: F, t3090: F, t1043: F, t3075: F, t1045: F, t3117: F, t3316: F, t994: F, t4891: F, t11659: F, t4910: F) -> (F, F, F, F, F, F, F, F) {
    let t11865 = t3046 * t1086;
    let t11866 = t11865 * t3090;
    let t11869 = t3075 * t1043;
    let t11870 = t11869 * t1045;
    let t11871 = t3117 * t11870;
    let t11874 = t994 * t3316;
    let t11875 = t11874 * t4891;
    let t11876 = t11659 * t4910;
    (t11865, t11866, t11869, t11870, t11871, t11874, t11875, t11876)
}
