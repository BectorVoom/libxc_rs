//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 852/1170 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk852<F: Float>(t11133: F, t11134: F, t11136: F, t11138: F, t11140: F, t11147: F, t11153: F, t11158: F, t11162: F, t11167: F, t11171: F, t996: F, t1096: F, t3325: F, t3269: F, t3075: F) -> (F, F, F, F) {
    let t11173 = -t11133 - 0.19755555555555555556e-1 * t11134 + 0.9877777777777777778e-2 * t11136 - 0.29633333333333333334e-1 * t11138 + 0.14816666666666666667e-1 * t11140 - 0.16462962962962962963e-1 * t11147 + 0.59266666666666666668e-1 * t11153 - 0.29633333333333333334e-1 * t11158 - 0.88900000000000000002e-1 * t11162 + 0.88900000000000000002e-1 * t11167 - 0.14816666666666666667e-1 * t11171;
    let t11174 = t996 * t11173;
    let t11177 = t1096 * t3325;
    let t11178 = t3269 * t11177;
    let t11183 = t3075 * t1096;
    (t11173, t11174, t11178, t11183)
}
