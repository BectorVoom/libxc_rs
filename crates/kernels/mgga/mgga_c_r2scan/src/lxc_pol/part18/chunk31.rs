//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 31/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk31<F: Float>(t12: F, t18: F, t26: F, t15: F) -> (F, F, F, F, F, F, F) {
    let t65 = F::cast_from(0.1549425e1_f64) * t12;
    let t66 = F::cast_from(0.420775e0_f64) * t18;
    let t67 = F::cast_from(0.1562925e0_f64) * t26;
    let t68 = F::cast_from(0.705945e1_f64) * t15 + t65 + t66 + t67;
    let t71 = F::cast_from(1.0_f64) + F::cast_from(0.32163958997385070134e2_f64) / t68;
    let t72 = F::ln(t71);
    let t76 = F::cast_from(1.0_f64) + F::cast_from(0.278125e-1_f64) * t12;
    (t65, t66, t67, t68, t71, t72, t76)
}
