//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 335/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk335(t2530: f64, t836: f64, t568: f64, t769: f64, t954: f64, t314: f64, t313: f64, t2013: f64, t970: f64, t2465: f64, t325: f64, t2464: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t2710 = t836 * t2530;
    let t2711 = t568 * t2710;
    let t2714 = t769 * t954;
    let t2717 = t314 * t2530;
    let t2718 = t313 * t2717;
    let t2721 = t2013 * t970;
    let t2723 = t2465 * t325;
    let t2724 = t2464 * t2723;
    (t2711, t2714, t2717, t2718, t2721, t2724)
}
