//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1365/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1365<F: Float>(t14005: F, t9816: F, t2713: F, t3964: F, t5617: F, t5686: F, t9744: F, t221: F, t4019: F, t5659: F, t4018: F, t3989: F, t5629: F) -> (F, F, F, F, F, F) {
    let t14007 = F::cast_from(0.25410001404642664112e-4_f64) * t9816 * t14005;
    let t14013 = t3964 * t2713 * t5617;
    let t14024 = F::cast_from(7.0_f64) / F::cast_from(24.0_f64) * t9744 * t5686;
    let t14036 = t4019 * t221 * t5659;
    let t14038 = F::cast_from(0.25410001404642664112e-4_f64) * t4018 * t14036;
    let t14040 = F::cast_from(0.40015750243531754508e-1_f64) * t3989 * t5629;
    (t14007, t14013, t14024, t14036, t14038, t14040)
}
