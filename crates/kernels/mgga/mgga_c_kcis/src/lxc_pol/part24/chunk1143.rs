//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1143/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1143<F: Float>(t100378: F, t100383: F, t100386: F, t100398: F, t100401: F, t101047: F, t101053: F, t18443: F, t26748: F, t27812: F, t29011: F, t4947: F, t7703: F, t7704: F, t95535: F, t95852: F, t95855: F) -> (F,) {
    let t101189 = -0.37134344353515625e-4 * t27812 * t101053 + 0.49555782539766601562e-5 * t95535 * t101047 - 0.44218518518518518517e-2 * t100378 + 0.99491666666666666664e-2 * t100383 - 0.11054629629629629629e-2 * t100386 + 0.41188271604938271605e-3 * t95852 + 0.10297067901234567901e-3 * t95855 - 0.23168402777777777778e-3 * t7703 * t4947 * t7704 * t18443 - 0.30891203703703703704e-3 * t26748 * t29011 + 0.99491666666666666664e-2 * t100398 + 0.33163888888888888888e-2 * t100401;
    (t101189,)
}
