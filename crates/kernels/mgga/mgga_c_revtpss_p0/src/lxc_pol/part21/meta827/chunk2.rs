//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3082/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3082<F: Float>(t56228: F, t43858: F, t43865: F, t43883: F, t43888: F, t43890: F, t43892: F, t43894: F, t43896: F, t56212: F, t56214: F, t56216: F, t56221: F, t56226: F, t56230: F, t56234: F, t56236: F, t56248: F, t56252: F, t56256: F) -> F {
    let t56360 = F::cast_from(0.19755555555555555556e-1_f64) * t56228;
    let t56375 = F::cast_from(0.98777777777777777781e-2_f64) * t56212 + F::cast_from(0.59266666666666666669e-1_f64) * t56214 - F::cast_from(0.16462962962962962963e-1_f64) * t56216 + F::cast_from(0.4938888888888888889e-1_f64) * t56221 + F::cast_from(0.88900000000000000002e-1_f64) * t56226 + t56360 - F::cast_from(0.14816666666666666667e-1_f64) * t56230 + F::cast_from(0.14816666666666666667e-1_f64) * t56234 - F::cast_from(0.15365432098765432099e-1_f64) * t56236 - F::cast_from(0.54876543209876543212e-2_f64) * t43858 - F::cast_from(0.13170370370370370371e-1_f64) * t43865 + F::cast_from(0.19755555555555555556e-1_f64) * t43883 - F::cast_from(0.46096296296296296298e-1_f64) * t43888 + F::cast_from(0.19755555555555555556e-1_f64) * t43890 + F::cast_from(0.39511111111111111113e-1_f64) * t43892 - F::cast_from(0.29633333333333333334e-1_f64) * t43894 - F::cast_from(0.4938888888888888889e-2_f64) * t43896 + F::cast_from(0.49388888888888888889e-1_f64) * t56248 + F::new(0.2667e0) * t56252 - F::cast_from(0.17780000000000000001e0_f64) * t56256;
    t56375
}
