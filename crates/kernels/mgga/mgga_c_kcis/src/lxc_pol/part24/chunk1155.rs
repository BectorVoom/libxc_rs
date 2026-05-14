//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1155/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1155<F: Float>(t101501: F, t2173: F, t100704: F, t100707: F, t100736: F, t100741: F, t100749: F, t100762: F, t1250: F, t2175: F, t70994: F, t93606: F, t96264: F, t96270: F, t100765: F, t100768: F, t100778: F, t100781: F, t100790: F, t101101: F, t101376: F, t7703: F, t93145: F, t93425: F, t93628: F, t96273: F, t96281: F) -> (F, F) {
    let t101502 = t2173 * t101501;
    let t101509 = -0.49745833333333333332e-2 * t100704 - 0.55273148148148148147e-2 * t100707 - 0.22109259259259259258e-2 * t100736 - 0.7369753086419753086e-3 * t100741 + t96264 + 0.15445601851851851852e-3 * t93606 + 0.16581944444444444444e-2 * t100749 + 0.23168402777777777778e-3 * t101502 - 0.69505208333333333333e-3 * t70994 * t1250 * t2175 - 0.88437037037037037035e-2 * t96270 + 0.66327777777777777776e-2 * t100762;
    let t101522 = -0.6183646701388888889e-4 * t93425 * t101376 - 0.22109259259259259259e-2 * t100765 + 0.66327777777777777776e-2 * t100768 - 0.55273148148148148147e-3 * t93145 - 0.33163888888888888888e-2 * t100778 + 0.44218518518518518516e-2 * t100781 + 0.27802083333333333334e-2 * t7703 * t101101 - 0.11054629629629629629e-2 * t96273 + t93628 + t96281 - 0.22109259259259259259e-2 * t100790;
    (t101509, t101522)
}
