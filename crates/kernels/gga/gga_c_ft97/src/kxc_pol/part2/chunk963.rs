//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 963/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk963<F: Float>(t14902: F, t1775: F, t4203: F, t4207: F, t4200: F, t13309: F, t4199: F, t10580: F, t2: F, t13315: F, t13320: F, t14624: F, t2771: F) -> (F, F, F, F, F, F, F, F) {
    let t14951 = F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t14902;
    let t14953 = F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1775 * t4203;
    let t14955 = F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t1775 * t4207;
    let t14957 = F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t1775 * t4200;
    let t14958 = t4199 * t13309;
    let t14961 = t10580 * t2;
    let t14962 = t14961 * t13315;
    let t14965 = t4199 * t13320;
    let t14968 = t2771 * t14624;
    (t14951, t14953, t14955, t14957, t14958, t14962, t14965, t14968)
}
