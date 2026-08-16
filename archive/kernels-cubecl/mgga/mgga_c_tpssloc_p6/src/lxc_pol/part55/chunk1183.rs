//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1183/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1183<F: Float>(t30714: F, t4191: F, t112818: F, t112820: F, t112829: F, t112835: F, t112841: F, t112846: F, t112851: F, t112856: F, t118586: F, t118588: F, t118590: F, t118592: F, t118594: F, t118596: F, t118602: F, t118606: F, t118608: F, t118610: F) -> F {
    let t118612 = t30714 * t4191;
    let t118615 = F::cast_from(0.13457585364713463618e-3_f64) * t118586 + F::cast_from(7.0_f64) / F::cast_from(576.0_f64) * t118588 - t118590 / F::cast_from(384.0_f64) - t118592 / F::cast_from(384.0_f64) - t118594 / F::cast_from(384.0_f64) + F::cast_from(7.0_f64) / F::cast_from(2304.0_f64) * t118596 + F::cast_from(0.80745512188280781708e-3_f64) * t112818 + F::cast_from(7.0_f64) / F::cast_from(576.0_f64) * t112820 + F::cast_from(0.56521858531796547196e-2_f64) * t112829 - F::cast_from(7.0_f64) / F::cast_from(2304.0_f64) * t118602 + t112835 - t112841 - F::cast_from(0.48447307312968469025e-2_f64) * t118606 - t118608 / F::cast_from(1536.0_f64) + t118610 / F::cast_from(384.0_f64) + t118612 / F::cast_from(384.0_f64) - F::cast_from(7.0_f64) / F::cast_from(2304.0_f64) * t112846 + t112851 + t112856;
    t118615
}
