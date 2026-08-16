//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1131/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1131(t197: f64, t8991: f64, t9606: f64, t274: f64, t41622: f64, t9600: f64, t10308: f64, t10309: f64, t10326: f64, t10328: f64, t10329: f64, t14844: f64, t2014: f64, t231: f64, t2344: f64, t2380: f64, t2394: f64, t2440: f64, t2697: f64, t2710: f64, t278: f64, t36827: f64, t39922: f64, t39926: f64, t41621: f64, t41627: f64, t41673: f64, t41762: f64, t43236: f64, t43707: f64, t43712: f64, t43715: f64, t43726: f64, t43732: f64, t43736: f64, t683: f64, t807: f64, t8948: f64, t8963: f64, t9525: f64) -> f64 {
    let t43742 = t8991 / t197 / t9606;
    let t43752 = t41622 * t274;
    let t43755 = t274 * t9600;
    let t43766 = -0.79692916077817074549e-2_f64 * t2014 * t231 * t41627 * t274 + 0.68769182700451188138e-1_f64 * t41673 * t43707 + 0.13302972333265952938e0_f64 * t43712 * t43707 + 0.959348966341294683e-1_f64 * t2710 * t43715 + 0.41932428475884870816e-1_f64 * t2394 * t43715 + 0.22136921132726965153e-3_f64 * t39926 * t2344 * t9525 * t10328 - 0.59031789687271907074e-3_f64 * t39922 * t10329 - 0.8854768453090786061e-3_f64 * t8963 * t10326 * t43726 - 0.11806357937454381415e-2_f64 * t8963 * t2440 * t2380 * t43732 + 0.43406294696984965172e-2_f64 * t8963 * t43736 * t14844 + 0.32991033661753008702e-2_f64 * t43742 * t43707 - 0.532971647967385935e-1_f64 * t807 * t41762 * t278 + 0.24033558310605691936e-3_f64 * t41621 * t43707 + 0.105346283539551678e1_f64 * t43236 * t10309 - 0.15647667480826127546e-2_f64 * t36827 * t43752 - 0.46820570462022968e0_f64 * t10308 * t43755 - 0.36171912247487470976e-3_f64 * t2014 * t231 * t41622 * t2697 * t274 - 0.49254336522043865661e-4_f64 * t8948 * t683 * t43752;
    t43766
}
