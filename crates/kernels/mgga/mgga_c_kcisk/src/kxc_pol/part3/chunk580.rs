//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 580/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk580<F: Float>(t1248: F, t1636: F, t4889: F, t1774: F, t24: F, t4640: F, t1720: F, t4644: F, t4648: F, t4838: F, t4842: F, t4845: F, t4848: F, t4866: F, t4874: F, t4876: F, t4882: F, t4884: F, t4888: F) -> (F, F, F, F, F, F) {
    let t4891 = t1248 * t4889 * t1636;
    let t4893 = t24 * t1774;
    let t4895 = t1248 * t4893 * t4640;
    let t4898 = t1248 * t1720 * t4644;
    let t4901 = t1248 * t1720 * t4648;
    let t4903 = -F::cast_from(0.9494625e0_f64) * t4866 + F::cast_from(0.1898925e1_f64) * t4874 + t4876 + F::cast_from(0.19931111111111111111e0_f64) * t4838 - F::cast_from(0.19931111111111111111e0_f64) * t4842 + F::cast_from(0.59793333333333333334e0_f64) * t4845 - F::cast_from(0.29896666666666666667e0_f64) * t4848 + F::cast_from(0.15358125e0_f64) * t4882 + F::cast_from(0.3071625e0_f64) * t4884 + t4888 + F::cast_from(0.21908444444444444444e0_f64) * t4891 - F::cast_from(0.5477111111111111111e-1_f64) * t4895 + F::cast_from(0.32862666666666666666e0_f64) * t4898 - F::cast_from(0.16431333333333333333e0_f64) * t4901;
    (t4891, t4893, t4895, t4898, t4901, t4903)
}
