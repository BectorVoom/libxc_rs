//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 873/1092 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk873<F: Float>(t30671: F, t30714: F, t30728: F, t30882: F, t30886: F, t30889: F, t30904: F, t30907: F, t30920: F, t30989: F, t31001: F, t31015: F, t31020: F, t31022: F, t31036: F, t31226: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t32545 = 0.10566559009306995095e0 * t30671;
    let t32557 = 0.2546875e-1 * t30714;
    let t32561 = 0.11321313224257494745e-1 * t30728;
    let t32619 = 0.21881628506185221314e-1 * t30882;
    let t32621 = 0.85748036236139473944e-3 * t30886;
    let t32622 = 0.25724410870841842183e-2 * t30889;
    let t32627 = 0.51448821741683684367e-2 * t30904;
    let t32628 = 0.24009450146119052704e-1 * t30907;
    let t32635 = 0.83861579438944405516e-2 * t30920;
    let t32664 = 0.57165357490759649297e-2 * t30989;
    let t32668 = 0.24009450146119052704e-1 * t31001;
    let t32670 = 0.7145669686344956162e-3 * t31015;
    let t32671 = 0.10482697429868050689e-2 * t31020;
    let t32672 = 0.12004725073059526352e-1 * t31022;
    let t32677 = 311.0 / 432.0 * t31036;
    let t32739 = 0.51448821741683684367e-2 * t31226;
    (t32545, t32557, t32561, t32619, t32621, t32622, t32627, t32628, t32635, t32664, t32668, t32670, t32671, t32672, t32677, t32739)
}
