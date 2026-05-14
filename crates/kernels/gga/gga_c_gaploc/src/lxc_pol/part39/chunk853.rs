//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 853/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk853<F: Float>(t42985: F, t32357: F, t5539: F, t9647: F, t32436: F, t13212: F, t7137: F, t13203: F, t32190: F, t935: F, t2508: F, t2580: F, t13209: F, t7129: F, t3431: F) -> (F, F, F, F, F, F, F, F, F) {
    let t42986 = 0.12817543716903707139e-2 * t42985;
    let t42988 = t9647 * t5539 * t32357;
    let t42989 = 0.12817543716903707139e-2 * t42988;
    let t42991 = t9647 * t5539 * t32436;
    let t42992 = 0.12817543716903707139e-2 * t42991;
    let t42998 = 0.30762104920568897135e-1 * t7137 * t13212;
    let t42999 = t7137 * t13203;
    let t43001 = t32190 * t935;
    let t43003 = t2508 * t2580 * t43001;
    let t43006 = 0.76905262301422242837e-2 * t7129 * t13209;
    let t43007 = t3431 * t935;
    (t42986, t42989, t42992, t42998, t42999, t43001, t43003, t43006, t43007)
}
