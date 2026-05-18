//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 988/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk988<F: Float>(t40588: F, t40591: F, t40596: F, t40599: F, t40602: F, t13194: F, t29439: F, t32357: F, t5539: F, t9647: F, t32436: F, t13212: F, t7137: F) -> (F, F, F, F, F, F, F, F, F) {
    let t42980 = F::new(0.1922631557535556071e-2) * t40588;
    let t42981 = F::new(0.4486140300916297499e-2) * t40591;
    let t42982 = F::new(0.7690526230142224284e-2) * t40596;
    let t42983 = F::new(0.3845263115071112142e-2) * t40599;
    let t42984 = F::new(0.1281754371690370714e-2) * t40602;
    let t42985 = t29439 * t13194;
    let t42986 = F::new(0.12817543716903707139e-2) * t42985;
    let t42988 = t9647 * t5539 * t32357;
    let t42989 = F::new(0.12817543716903707139e-2) * t42988;
    let t42991 = t9647 * t5539 * t32436;
    let t42992 = F::new(0.12817543716903707139e-2) * t42991;
    let t42998 = F::new(0.30762104920568897135e-1) * t7137 * t13212;
    (t42980, t42981, t42982, t42983, t42984, t42986, t42989, t42992, t42998)
}
