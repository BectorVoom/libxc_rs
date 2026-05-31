//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 872/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk872<F: Float>(t13083: F, t13098: F, t1203: F, t1212: F, t12885: F, t3722: F, t12974: F, t12922: F, t12927: F, t12929: F, t12931: F, t12933: F, t12948: F, t12954: F, t12959: F, t12985: F, t12989: F) -> (F, F, F, F) {
    let t13099 = t13083 + t13098;
    let t13101 = t1203 * t13099 * t1212;
    let t13105 = t3722 * t12885 * t1212;
    let t13110 = F::cast_from(0.55403703703703703703e-1_f64) * t12974;
    let t13121 = -t13110 - F::cast_from(0.23744444444444444444e-1_f64) * t12929 + F::cast_from(0.11872222222222222222e-1_f64) * t12933 - F::cast_from(0.35616666666666666666e-1_f64) * t12948 + F::cast_from(0.17808333333333333333e-1_f64) * t12931 - F::cast_from(0.19787037037037037037e-1_f64) * t12922 + F::cast_from(0.71233333333333333332e-1_f64) * t12954 - F::cast_from(0.35616666666666666666e-1_f64) * t12985 - F::cast_from(0.10685e0_f64) * t12959 + F::cast_from(0.10685e0_f64) * t12989 - F::cast_from(0.17808333333333333333e-1_f64) * t12927;
    (t13099, t13101, t13105, t13121)
}
