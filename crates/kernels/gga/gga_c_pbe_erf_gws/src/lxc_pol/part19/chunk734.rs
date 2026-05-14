//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 734/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk734<F: Float>(t1343: F, t169: F, t700: F, t1383: F, t766: F, t770: F, t289: F, t4598: F, t274: F, t413: F, t39: F, t745: F, t1553: F, t2718: F, t2704: F, t502: F) -> (F, F, F, F, F, F, F, F) {
    let t5713 = t169 * t1343 * t700;
    let t5717 = 0.15917832887339686635e0 * t169 * t766 * t1383;
    let t5726 = t169 * t770 * t1383;
    let t5730 = 0.31835665774679373271e-1 * t169 * t289 * t4598;
    let t5732 = 0.12798016258123051272e1 * t413 * t274;
    let t5733 = t39 * t745;
    let t5749 = 0.65290666666666666667e0 * t1553 * t2718;
    let t5751 = 0.76172444444444444444e0 * t502 * t2704;
    (t5713, t5717, t5726, t5730, t5732, t5733, t5749, t5751)
}
