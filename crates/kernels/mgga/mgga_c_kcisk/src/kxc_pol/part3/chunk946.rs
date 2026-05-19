//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 946/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk946<F: Float>(t3883: F, t965: F, t13987: F, t13989: F, t13991: F, t13993: F, t13995: F, t13998: F, t14001: F, t14003: F, t14005: F, t14008: F, t158: F, t173: F) -> F {
    let t14011 = t965 * t3883;
    let t14013 = -F::new(0.28104e-1) * t13987 - F::new(0.32788e-1) * t13989 - F::cast_from(0.352891875e-4_f64) * t13991 + F::new(0.4705225e-4) * t13993 + F::new(0.50413125e-5) * t173 * t13995 + F::cast_from(0.22405833333333333333e-5_f64) * t173 * t13998 + F::new(0.14052e-1) * t14001 - F::new(0.4684e-2) * t14003 - F::new(0.3513e-2) * t158 * t14005 + F::cast_from(0.78066666666666666667e-3_f64) * t158 * t14008 - F::cast_from(0.39624999999999999999e-2_f64) * t14011;
    t14013
}
