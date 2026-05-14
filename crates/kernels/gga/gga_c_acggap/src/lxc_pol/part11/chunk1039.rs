//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1039/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1039<F: Float>(t4640: F, t7332: F, t4645: F, t570: F, t1507: F, t2020: F, t30120: F, t8793: F, t1165: F, t33735: F, t604: F, t7413: F, t8948: F, t4680: F, t8947: F, t1181: F, t2068: F, t20972: F) -> (F, F, F, F, F, F, F, F) {
    let t36147 = t7332 * t4640;
    let t36149 = t570 * t4645;
    let t36151 = t2020 * t1507;
    let t36152 = 7.0 / 144.0 * t36151;
    let t36156 = t30120 * t8793;
    let t36157 = 0.62896184579208304136e-3 * t36156;
    let t36160 = t7413 * t1165 * t604 * t33735;
    let t36162 = t30120 * t8948;
    let t36163 = 0.42874018118069736972e-3 * t36162;
    let t36165 = t7413 * t4680 * t8947;
    let t36169 = t2068 * t1181 * t604 * t20972;
    (t36147, t36149, t36152, t36157, t36160, t36163, t36165, t36169)
}
