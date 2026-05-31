//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 798/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk798<F: Float>(t12225: F, t12318: F, t2029: F, t1990: F, t5444: F, t11222: F, t11231: F, t11233: F, t11239: F, t11241: F, t11453: F, t11456: F, t11650: F, t11652: F, t11661: F, t11663: F, t11669: F, t11674: F, t11680: F, t11685: F, t11967: F, t1994: F, t5440: F) -> F {
    let t12319 = t12225 + t12318;
    let t12320 = t12319 * t2029;
    let t12325 = t1990 * t5444;
    let t12338 = F::cast_from(0.38691203703703703703e-2_f64) * t11222 - F::cast_from(0.10446625e-1_f64) * t11231 - F::cast_from(0.77382407407407407405e-3_f64) * t11233 + F::cast_from(0.10446625e-1_f64) * t11239 + F::cast_from(0.69644166666666666665e-2_f64) * t11241 - F::cast_from(0.193e0_f64) * t1994 * t12320 - F::cast_from(0.386e0_f64) * t1994 * t11967 + F::cast_from(0.223494e0_f64) * t12325 * t5440 - F::cast_from(0.17411041666666666666e-2_f64) * t11453 + F::cast_from(0.34822083333333333333e-2_f64) * t11456 + F::cast_from(0.17411041666666666666e-2_f64) * t11650 + F::cast_from(0.34822083333333333333e-2_f64) * t11652 - F::cast_from(0.52233124999999999998e-2_f64) * t11661 - F::cast_from(0.69644166666666666665e-2_f64) * t11663 - F::cast_from(0.69644166666666666665e-2_f64) * t11669 + F::cast_from(0.10446625e-1_f64) * t11674 - F::cast_from(0.77382407407407407405e-3_f64) * t11680 - F::cast_from(0.46429444444444444443e-2_f64) * t11685;
    t12338
}
