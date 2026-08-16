//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 812/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk812<F: Float>(t8694: F, t8706: F, t8710: F, t8712: F, t8714: F, t8716: F, t8718: F, t8722: F, t8742: F, t8744: F, t8772: F, t8829: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t9254 = F::cast_from(0.17149607247227894789e-2_f64) * t8694;
    let t9261 = F::cast_from(0.17149607247227894789e-2_f64) * t8706;
    let t9263 = F::cast_from(0.34299214494455789578e-2_f64) * t8710;
    let t9264 = F::cast_from(0.80031500487063509015e-2_f64) * t8712;
    let t9265 = F::cast_from(0.80031500487063509015e-2_f64) * t8714;
    let t9266 = F::cast_from(0.16006300097412701803e-1_f64) * t8716;
    let t9267 = F::cast_from(0.34299214494455789578e-2_f64) * t8718;
    let t9269 = F::cast_from(0.12862205435420921092e-2_f64) * t8722;
    let t9277 = F::cast_from(0.4584375e-1_f64) * t8742;
    let t9278 = F::cast_from(0.305625e-1_f64) * t8744;
    let t9292 = F::cast_from(0.1528125e-1_f64) * t8772;
    let t9309 = F::cast_from(0.84046875e-1_f64) * t8829;
    (t9254, t9261, t9263, t9264, t9265, t9266, t9267, t9269, t9277, t9278, t9292, t9309)
}
