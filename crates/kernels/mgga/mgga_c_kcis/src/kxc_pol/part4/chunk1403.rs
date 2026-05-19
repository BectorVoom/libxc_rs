//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1403/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1403<F: Float>(t18090: F, t18144: F, t18182: F, t18225: F, t1616: F, t12263: F, t12271: F, t12275: F, t12277: F, t12279: F, t12303: F, t12307: F, t12915: F, t1592: F, t16797: F, t16800: F, t16812: F, t16820: F, t16997: F, t18056: F, t18059: F, t18060: F, t18061: F, t4409: F, t6189: F) -> F {
    let t18227 = t18090 + t18144 + t18182 + t18225;
    let t18228 = t18227 * t1616;
    let t18237 = -t18056 - F::cast_from(0.19345601851851851852e-2_f64) * t16797 + F::cast_from(0.11607361111111111111e-2_f64) * t16800 - t18059 + t18060 - t18061 + F::cast_from(0.69644166666666666666e-2_f64) * t16812 - F::cast_from(0.25794135802469135802e-3_f64) * t12263 + F::cast_from(0.23214722222222222222e-2_f64) * t12271 - F::cast_from(0.51588271604938271604e-3_f64) * t12275 + F::cast_from(0.77382407407407407407e-3_f64) * t12277 + F::cast_from(0.77382407407407407406e-3_f64) * t12279 - F::new(0.66725e-1) * t1592 * t18228 - F::cast_from(0.77382407407407407406e-3_f64) * t12303 - F::cast_from(0.38691203703703703703e-3_f64) * t16820 + t12915 + F::cast_from(0.11607361111111111111e-2_f64) * t12307 - F::new(0.13345e0) * t4409 * t6189 + F::cast_from(0.23214722222222222222e-2_f64) * t16997;
    t18237
}
