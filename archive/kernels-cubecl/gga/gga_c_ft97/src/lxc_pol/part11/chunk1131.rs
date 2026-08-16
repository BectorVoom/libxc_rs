//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1131/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1131<F: Float>(t197: F, t8991: F, t9606: F, t274: F, t41622: F, t9600: F, t10308: F, t10309: F, t10326: F, t10328: F, t10329: F, t14844: F, t2014: F, t231: F, t2344: F, t2380: F, t2394: F, t2440: F, t2697: F, t2710: F, t278: F, t36827: F, t39922: F, t39926: F, t41621: F, t41627: F, t41673: F, t41762: F, t43236: F, t43707: F, t43712: F, t43715: F, t43726: F, t43732: F, t43736: F, t683: F, t807: F, t8948: F, t8963: F, t9525: F) -> F {
    let t43742 = t8991 / t197 / t9606;
    let t43752 = t41622 * t274;
    let t43755 = t274 * t9600;
    let t43766 = -F::cast_from(0.79692916077817074549e-2_f64) * t2014 * t231 * t41627 * t274 + F::cast_from(0.68769182700451188138e-1_f64) * t41673 * t43707 + F::cast_from(0.13302972333265952938e0_f64) * t43712 * t43707 + F::cast_from(0.959348966341294683e-1_f64) * t2710 * t43715 + F::cast_from(0.41932428475884870816e-1_f64) * t2394 * t43715 + F::cast_from(0.22136921132726965153e-3_f64) * t39926 * t2344 * t9525 * t10328 - F::cast_from(0.59031789687271907074e-3_f64) * t39922 * t10329 - F::cast_from(0.8854768453090786061e-3_f64) * t8963 * t10326 * t43726 - F::cast_from(0.11806357937454381415e-2_f64) * t8963 * t2440 * t2380 * t43732 + F::cast_from(0.43406294696984965172e-2_f64) * t8963 * t43736 * t14844 + F::cast_from(0.32991033661753008702e-2_f64) * t43742 * t43707 - F::cast_from(0.532971647967385935e-1_f64) * t807 * t41762 * t278 + F::cast_from(0.24033558310605691936e-3_f64) * t41621 * t43707 + F::cast_from(0.105346283539551678e1_f64) * t43236 * t10309 - F::cast_from(0.15647667480826127546e-2_f64) * t36827 * t43752 - F::cast_from(0.46820570462022968e0_f64) * t10308 * t43755 - F::cast_from(0.36171912247487470976e-3_f64) * t2014 * t231 * t41622 * t2697 * t274 - F::cast_from(0.49254336522043865661e-4_f64) * t8948 * t683 * t43752;
    t43766
}
