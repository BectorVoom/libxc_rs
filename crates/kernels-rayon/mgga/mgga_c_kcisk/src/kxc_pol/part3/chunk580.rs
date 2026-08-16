//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 580/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk580(t1248: f64, t1636: f64, t4889: f64, t1774: f64, t24: f64, t4640: f64, t1720: f64, t4644: f64, t4648: f64, t4838: f64, t4842: f64, t4845: f64, t4848: f64, t4866: f64, t4874: f64, t4876: f64, t4882: f64, t4884: f64, t4888: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4891 = t1248 * t4889 * t1636;
    let t4893 = t24 * t1774;
    let t4895 = t1248 * t4893 * t4640;
    let t4898 = t1248 * t1720 * t4644;
    let t4901 = t1248 * t1720 * t4648;
    let t4903 = -0.9494625e0_f64 * t4866 + 0.1898925e1_f64 * t4874 + t4876 + 0.19931111111111111111e0_f64 * t4838 - 0.19931111111111111111e0_f64 * t4842 + 0.59793333333333333334e0_f64 * t4845 - 0.29896666666666666667e0_f64 * t4848 + 0.15358125e0_f64 * t4882 + 0.3071625e0_f64 * t4884 + t4888 + 0.21908444444444444444e0_f64 * t4891 - 0.5477111111111111111e-1_f64 * t4895 + 0.32862666666666666666e0_f64 * t4898 - 0.16431333333333333333e0_f64 * t4901;
    (t4891, t4893, t4895, t4898, t4901, t4903)
}
