//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1034/1196 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1034<F: Float>(t2013: F, t8995: F, t2033: F, t9593: F, t116: F, t7741: F, t27240: F, t27246: F, t27251: F, t27254: F, t27256: F, t28034: F, t27924: F, t27926: F, t27929: F, t27937: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t28196 = t2013 * t8995;
    let t28197 = t2033 * t9593;
    let t28276 = t116 * t7741;
    let t28330 = 0.11433071498151929859e-3 * t27240;
    let t28333 = 7.0 / 72.0 * t27246;
    let t28335 = 0.2032800112371413129e-3 * t27251;
    let t28336 = 0.28582678745379824648e-4 * t27254;
    let t28337 = 0.16006300097412701803e-1 * t27256;
    let t28679 = 2.0 / 3.0 * t28034;
    let t28872 = 0.2032800112371413129e-3 * t27924;
    let t28873 = 0.16006300097412701803e-1 * t27926;
    let t28874 = 0.28582678745379824648e-4 * t27929;
    let t28877 = 0.11433071498151929859e-3 * t27937;
    (t28196, t28197, t28276, t28330, t28333, t28335, t28336, t28337, t28679, t28872, t28873, t28874, t28877)
}
