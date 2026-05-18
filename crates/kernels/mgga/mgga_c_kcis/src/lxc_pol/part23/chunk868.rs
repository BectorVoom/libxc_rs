//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 868/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk868<F: Float>(t16355: F, t16369: F, t1444: F, t1482: F, t16360: F, t1102: F, t11632: F, t15991: F, t15994: F, t15997: F, t16001: F, t16003: F, t16006: F, t16010: F, t16014: F, t16018: F, t16022: F, t16026: F, t16031: F, t16035: F, t16038: F, t16349: F, t16353: F, t16356: F, t16361: F, t16366: F, t486: F) -> F {
    let t16370 = t16369 * t16355;
    let t16373 = t1482 * t1444;
    let t16374 = t16373 * t16360;
    let t16377 = F::new(0.19711289e-2) * t1102 * t15991 + F::new(0.21901432222222222221e-2) * t15994 - F::new(0.7391733375e-3) * t1102 * t15997 + t16001 - t16003 + F::new(0.1478346675e-2) * t1102 * t16006 + F::new(0.7391733375e-3) * t1102 * t16010 - F::new(0.19711289e-2) * t11632 * t16014 + F::new(0.26281718666666666666e-2) * t11632 * t16018 + F::new(0.98556445e-3) * t11632 * t16022 - F::new(0.19711289e-2) * t11632 * t16026 - F::new(0.295669335e-2) * t1102 * t16031 - F::new(0.1478346675e-2) * t1102 * t16035 - F::new(0.14600954814814814815e-3) * t16038 - F::new(4.0) * t486 * t16349 + F::new(0.32852148333333333333e-2) * t16353 * t16356 - F::new(0.21901432222222222222e-2) * t16353 * t16361 - F::new(0.19711289e-2) * t11632 * t16366 - F::new(0.39422578e-2) * t11632 * t16370 + F::new(0.26281718666666666666e-2) * t11632 * t16374;
    t16377
}
