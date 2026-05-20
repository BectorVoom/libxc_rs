//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1426/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1426<F: Float>(t13762: F, t13763: F, t13765: F, t13772: F, t13778: F, t22023: F, t22028: F, t22030: F, t9711: F, t9712: F, t9725: F, t9729: F) -> F {
    let t22035 = F::cast_from(0.71456696863449561619e-5_f64) * t22023 - F::cast_from(0.14291339372689912324e-4_f64) * t22028 + F::cast_from(0.40015750243531754507e-2_f64) * t22030 + t9711 - F::cast_from(0.30488190661738479624e-3_f64) * t9712 + t9725 - t9729 - t13762 + F::cast_from(0.80031500487063509015e-2_f64) * t13763 + F::cast_from(0.10841600599314203355e-2_f64) * t13765 - t13772 + t13778;
    t22035
}
