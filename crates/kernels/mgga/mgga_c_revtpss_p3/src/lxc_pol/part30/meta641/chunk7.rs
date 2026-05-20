//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2235/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2235<F: Float>(t17303: F, t7613: F, t29062: F, t3678: F, t17209: F, t26880: F, t29019: F, t3707: F, t26873: F, t5265: F, t15687: F, t26865: F) -> (F, F, F, F, F, F) {
    let t104825 = t7613 * t17303;
    let t104828 = F::cast_from(0.30488190661738479624e-2_f64) * t29062 * t3678;
    let t104833 = F::cast_from(0.3811023832717309953e-3_f64) * t26880 * t17209;
    let t104834 = t3707 * t29019;
    let t104844 = F::cast_from(0.57165357490759649296e-3_f64) * t26873 * t5265;
    let t104852 = t26865 * t15687;
    (t104825, t104828, t104833, t104834, t104844, t104852)
}
