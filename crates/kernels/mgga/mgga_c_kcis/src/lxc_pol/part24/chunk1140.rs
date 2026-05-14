//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1140/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1140<F: Float>(t100204: F, t100208: F, t100212: F, t100219: F, t100229: F, t100983: F, t101084: F, t101101: F, t26685: F, t26695: F, t27812: F, t27826: F, t27832: F, t71203: F, t7703: F, t95764: F, t9933: F) -> (F,) {
    let t101104 = -0.33163888888888888888e-2 * t100204 - 0.185671721767578125e-4 * t27812 * t101084 + 0.55273148148148148147e-3 * t100208 + 0.73697530864197530862e-3 * t100212 - 0.33163888888888888888e-2 * t100219 - 0.36848765432098765431e-3 * t100229 + 0.92673611111111111112e-3 * t27832 * t27826 + 0.30891203703703703704e-3 * t7703 * t9933 * t26695 * t71203 - 0.20594135802469135803e-3 * t95764 + 0.37101880208333333334e-3 * t26685 * t100983 + 0.55652820312500000001e-3 * t26685 * t101101;
    (t101104,)
}
