//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 659/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk659<F: Float>(t10999: F, t1248: F, t4640: F, t24: F, t5005: F, t10464: F, t10450: F, t1720: F, t10791: F, t1636: F, t10585: F, t4893: F, t10593: F, t10937: F, t10941: F, t10957: F, t10963: F, t10988: F, t10991: F, t10994: F, t10997: F) -> (F, F, F, F, F, F, F) {
    let t11001 = t1248 * t10999 * t4640;
    let t11003 = t24 * t5005;
    let t11005 = t1248 * t11003 * t10464;
    let t11008 = t1248 * t1720 * t10450;
    let t11013 = t1248 * t10791 * t1636;
    let t11016 = t1248 * t4893 * t10585;
    let t11019 = t1248 * t1720 * t10593;
    let t11023 = -0.65725333333333333332e0 * t10988 + 0.32862666666666666666e0 * t10991 - 0.98587999999999999998e0 * t10994 + 0.32862666666666666666e0 * t10997 + 0.10954222222222222222e0 * t11001 - 0.73028148148148148146e-1 * t11005 - 0.16431333333333333333e0 * t11008 - 0.59793333333333333333e0 * t10957 + 0.17938e1 * t10963 - 0.5477111111111111111e0 * t11013 - 0.16431333333333333333e0 * t11016 + 0.98587999999999999998e0 * t11019 - 0.39862222222222222223e0 * t10937 + 0.19931111111111111111e0 * t10941;
    (t11001, t11005, t11008, t11013, t11016, t11019, t11023)
}
