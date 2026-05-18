//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 713/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk713<F: Float>(t11023: F, t11065: F, t1725: F, t10937: F, t10941: F, t10957: F, t10963: F, t10988: F, t10991: F, t10994: F, t10997: F, t11001: F, t11005: F, t11008: F, t11013: F, t11016: F, t11019: F) -> (F, F) {
    let t11066 = t11023 + t11065;
    let t11067 = t11066 * t1725;
    let t11084 = -F::new(0.66228e0) * t10988 + F::new(0.33114e0) * t10991 - F::new(0.99342e0) * t10994 + F::new(0.33114e0) * t10997 + F::new(0.11038e0) * t11001 - F::new(0.73586666666666666666e-1) * t11005 - F::new(0.16557e0) * t11008 - F::new(0.60384999999999999999e0) * t10957 + F::new(0.181155e1) * t10963 - F::new(0.5519e0) * t11013 - F::new(0.16557e0) * t11016 + F::new(0.99342e0) * t11019 - F::new(0.40256666666666666668e0) * t10937 + F::new(0.20128333333333333333e0) * t10941;
    (t11067, t11084)
}
