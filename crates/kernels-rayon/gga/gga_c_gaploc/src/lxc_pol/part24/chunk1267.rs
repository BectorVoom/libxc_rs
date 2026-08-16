//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1267/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1267(t10627: f64, t1880: f64, t23335: f64, t6066: f64, t1710: f64) -> (f64, f64, f64) {
    let t32889 = t10627 * t1880;
    let t32892 = 0.14300195980740170668e1_f64 * t23335 * t6066 * t32889;
    let t32893 = t10627 * t1710;
    (t32889, t32892, t32893)
}
