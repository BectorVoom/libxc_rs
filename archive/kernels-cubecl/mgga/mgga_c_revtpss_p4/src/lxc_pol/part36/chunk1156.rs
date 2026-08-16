//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1156/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1156<F: Float>(t116: F, t7741: F, t27240: F, t27246: F, t27251: F, t27254: F, t27256: F, t28034: F, t27924: F, t27926: F, t27929: F, t27937: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t28276 = t116 * t7741;
    let t28330 = F::cast_from(0.11433071498151929859e-3_f64) * t27240;
    let t28333 = F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t27246;
    let t28335 = F::cast_from(0.2032800112371413129e-3_f64) * t27251;
    let t28336 = F::cast_from(0.28582678745379824648e-4_f64) * t27254;
    let t28337 = F::cast_from(0.16006300097412701803e-1_f64) * t27256;
    let t28679 = F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t28034;
    let t28872 = F::cast_from(0.2032800112371413129e-3_f64) * t27924;
    let t28873 = F::cast_from(0.16006300097412701803e-1_f64) * t27926;
    let t28874 = F::cast_from(0.28582678745379824648e-4_f64) * t27929;
    let t28877 = F::cast_from(0.11433071498151929859e-3_f64) * t27937;
    (t28276, t28330, t28333, t28335, t28336, t28337, t28679, t28872, t28873, t28874, t28877)
}
