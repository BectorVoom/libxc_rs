//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1252/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1252(t32697: f64, t10679: f64, t10789: f64, t1897: f64, t29631: f64, t32669: f64, t32671: f64, t32674: f64, t32676: f64, t32679: f64, t32681: f64, t32683: f64, t32685: f64, t32691: f64, t32695: f64, t5227: f64, t5524: f64, t5836: f64) -> f64 {
    let t32698 = 0.96131577876777803547e-3_f64 * t32697;
    let t32701 = -0.8545029144602471425e-3_f64 * t5524 * t10679 - t32669 - t32671 + t32674 + t32676 + t29631 - t32679 - t32681 - t32683 + t32685 + 0.46143157380853345702e-1_f64 * t1897 * t10789 * t5836 + t32691 - t32695 - t32698 + 0.17090058289204942853e-2_f64 * t5227 * t10679;
    t32701
}
