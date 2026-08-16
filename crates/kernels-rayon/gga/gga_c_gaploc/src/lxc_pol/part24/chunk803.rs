//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 803/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk803(t7438: f64, t7585: f64, t314: f64, t7112: f64, t313: f64, t2154: f64, t954: f64, t2717: f64, t769: f64, t836: f64, t568: f64, t808: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t7586 = t7585 * t7438;
    let t7589 = t314 * t7112;
    let t7590 = t313 * t7589;
    let t7593 = t2154 * t954;
    let t7596 = t769 * t2717;
    let t7601 = t836 * t7112;
    let t7602 = t568 * t7601;
    let t7607 = t808 * t7112;
    (t7586, t7589, t7590, t7593, t7596, t7602, t7607)
}
