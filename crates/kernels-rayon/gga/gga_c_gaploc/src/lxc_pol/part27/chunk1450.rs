//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1450/1468 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1450(t12213: f64, t12255: f64, t12294: f64, t1897: f64, t1935: f64, t2580: f64, t32147: f64, t32149: f64, t32152: f64, t32154: f64, t32159: f64, t32161: f64, t32167: f64, t32169: f64, t32172: f64, t32185: f64, t3727: f64, t5397: f64, t5836: f64, t7129: f64) -> f64 {
    let t39375 = 0.30762104920568897134e-1_f64 * t7129 * t12294 - t32147 + t32149 + t32152 + t32154 + t32159 + t32161 + 0.46143157380853345702e-1_f64 * t1897 * t12255 * t5836 - 0.30762104920568897134e-1_f64 * t1897 * t2580 * t12213 * t5397 - t32167 + t32169 + t32172 - 0.76905262301422242837e-2_f64 * t1935 * t3727 - t32185;
    t39375
}
