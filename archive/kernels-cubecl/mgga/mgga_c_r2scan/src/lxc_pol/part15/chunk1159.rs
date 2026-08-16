//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1159/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1159<F: Float>(t10903: F, t11770: F, t2201: F, t37937: F, t37940: F, t37947: F, t37951: F, t37954: F, t37957: F, t39874: F, t39879: F, t39882: F, t39887: F, t39891: F) -> F {
    let t39894 = t2201 * t10903 * t11770;
    let t39895 = F::cast_from(0.46574606203128791246e-1_f64) * t39894;
    let t39896 = F::cast_from(0.47609969197673950972e-2_f64) * t37937 + F::cast_from(0.2600466522016280569e0_f64) * t39874 + F::cast_from(0.14282990759302185292e-1_f64) * t37940 + F::cast_from(0.31147743054556651236e-1_f64) * t37947 + F::cast_from(0.93443229163669953708e-1_f64) * t37951 + F::cast_from(0.21831846657716620896e-2_f64) * t39879 + F::cast_from(0.22511059664845582436e0_f64) * t39882 - t39887 + F::cast_from(0.71414953796510926458e-2_f64) * t37954 + F::cast_from(0.23804984598836975486e-2_f64) * t37957 + F::cast_from(0.21831846657716620896e-2_f64) * t39891 - t39895;
    t39896
}
