//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1052/1200 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1052<F: Float>(t32016: F, t372: F, t11922: F, t32021: F, t8522: F, t42793: F, t8523: F, t120361: F, t3057: F, t1035: F, t42066: F, t120322: F) -> (F, F, F, F, F, F) {
    let t120430 = t372 * t32016;
    let t120443 = t8522 * t32021 * t11922;
    let t120447 = F::new(0.41319254676723345357e-4) * t8522 * t8523 * t42793;
    let t120448 = t3057 * t120361;
    let t120452 = t42066 * t1035;
    let t120460 = t372 * t120322;
    (t120430, t120443, t120447, t120448, t120452, t120460)
}
