//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 710/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk710(t10791: f64, t1248: f64, t1636: f64, t10585: f64, t4893: f64, t10593: f64, t1720: f64, t10937: f64, t10941: f64, t10957: f64, t10963: f64, t10988: f64, t10991: f64, t10994: f64, t10997: f64, t11001: f64, t11005: f64, t11008: f64) -> (f64, f64, f64, f64) {
    let t11013 = t1248 * t10791 * t1636;
    let t11016 = t1248 * t4893 * t10585;
    let t11019 = t1248 * t1720 * t10593;
    let t11023 = -0.65725333333333333332e0_f64 * t10988 + 0.32862666666666666666e0_f64 * t10991 - 0.98587999999999999998e0_f64 * t10994 + 0.32862666666666666666e0_f64 * t10997 + 0.10954222222222222222e0_f64 * t11001 - 0.73028148148148148146e-1_f64 * t11005 - 0.16431333333333333333e0_f64 * t11008 - 0.59793333333333333333e0_f64 * t10957 + 0.17938e1_f64 * t10963 - 0.5477111111111111111e0_f64 * t11013 - 0.16431333333333333333e0_f64 * t11016 + 0.98587999999999999998e0_f64 * t11019 - 0.39862222222222222223e0_f64 * t10937 + 0.19931111111111111111e0_f64 * t10941;
    (t11013, t11016, t11019, t11023)
}
