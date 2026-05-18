//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 743/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk743<F: Float>(t172: F, t1948: F, t201: F, t755: F, t1953: F, t3318: F, t104: F, t1879: F, t1880: F, t1928: F, t3316: F, t3539: F, t606: F, t616: F, t6477: F, t6560: F, t6811: F, t6816: F, t6819: F, t6856: F, t714: F, t7142: F, t95: F) -> (F, F) {
    let t7153 = t172 * t1948;
    let t7157 = t755 * t201;
    let t7158 = t7157 * t1953;
    let t7159 = t3318 * t7158;
    let t7168 = F::new(0.25844881434903430496e-2) * t95 * t104 * t7142 * t714 + F::new(0.77534644304710291488e-2) * t95 * t606 * t6560 + F::new(0.23260393291413087447e-1) * t1879 * t1880 * t1948 + F::new(0.46520786582826174894e-1) * t3539 * t7153 * t616 + F::new(3.0) / F::new(2.0) * t3316 * t7159 + t6811 + F::new(0.15506928860942058298e-1) * t95 * t6856 * t172 + t6477 + t6816 + F::new(0.46520786582826174894e-1) * t3539 * t1880 * t1928 - t6819;
    (t7159, t7168)
}
