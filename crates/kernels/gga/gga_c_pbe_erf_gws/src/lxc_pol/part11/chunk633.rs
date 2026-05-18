//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 633/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk633<F: Float>(t169: F, t242: F, t5697: F, t1339: F, t700: F, t1383: F, t766: F, t289: F, t4598: F, t274: F, t413: F, t1553: F, t2718: F) -> (F, F, F, F, F, F) {
    let t5700 = F::new(0.5188034422540342311e0) * t169 * t5697 * t242;
    let t5707 = F::new(0.42447554366239164361e0) * t169 * t1339 * t700;
    let t5717 = F::new(0.15917832887339686635e0) * t169 * t766 * t1383;
    let t5730 = F::new(0.31835665774679373271e-1) * t169 * t289 * t4598;
    let t5732 = F::new(0.12798016258123051272e1) * t413 * t274;
    let t5749 = F::new(0.65290666666666666667e0) * t1553 * t2718;
    (t5700, t5707, t5717, t5730, t5732, t5749)
}
