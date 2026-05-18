//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 743/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk743<F: Float>(t1302: F, t32174: F, t1303: F, t22736: F, t22796: F, t3066: F, t32125: F, t32129: F, t32133: F, t32138: F, t32140: F, t32141: F, t32146: F, t32148: F, t32153: F, t32156: F, t32161: F, t32164: F, t32169: F, t32170: F, t429: F, t5533: F, t5587: F, t7172: F) -> (F, F) {
    let t32175 = t32174 * t1302;
    let t32178 = -F::new(0.76612330055555555556e-1) * t32125 * t1303 - F::new(0.76612330055555555556e-1) * t32129 * t1303 - F::new(0.22979081259345929704e-6) * t22736 * t32133 * t3066 + F::new(0.11738898233082762228e-1) * t32138 * t32140 * t32141 + F::new(0.89080607335887169333e-3) * t32146 * t32148 - F::new(0.39601100101559655353e-5) * t22796 * t32153 + F::new(4.0) * t32156 * t5533 - F::new(2.0) * t7172 * t429 + F::new(0.42300125954037691564e-4) * t32161 * t32164 - F::new(0.17608347349624143343e-1) * t32169 * t32140 * t32170 - F::new(0.27246626553445399075e-2) * t5587 * t32175;
    (t32175, t32178)
}
