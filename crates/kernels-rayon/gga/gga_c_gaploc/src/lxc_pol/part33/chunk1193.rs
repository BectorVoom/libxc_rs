//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1193/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1193(t24722: f64, t2508: f64, t2541: f64, t1897: f64, t2580: f64, t7068: f64, t8469: f64, t21455: f64, t2958: f64, t21460: f64, t3487: f64, t486: f64, t7069: f64) -> (f64, f64, f64, f64, f64) {
    let t32131 = 0.53833683610995569986e-1_f64 * t2508 * t2541 * t24722;
    let t32135 = 0.30762104920568897134e-1_f64 * t1897 * t2580 * t8469 * t7068;
    let t32139 = 0.30762104920568897134e-1_f64 * t1897 * t2580 * t2958 * t21455;
    let t32143 = 0.15381052460284448567e-1_f64 * t1897 * t2580 * t2958 * t21460;
    let t32145 = t3487 * t486 * t7069;
    (t32131, t32135, t32139, t32143, t32145)
}
