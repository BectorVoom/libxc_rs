//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 271/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk271<F: Float>(t330: F, t918: F, t101: F, t296: F, t299: F, t304: F, t308: F, t315: F, t316: F, t324: F, t333: F, t647: F, t654: F, t661: F, t665: F, t870: F, t871: F, t875: F, t880: F, t885: F, t890: F, t891: F, t895: F, t896: F, t899: F, t902: F, t903: F, t908: F, t914: F) -> (F, F) {
    let t919 = t330 * t918;
    let t930 = -F::new(0.125104062565404384e1) * t296 * t647 * t299 + F::new(0.58691349263882304531e0) * t870 * t654 * t871 + F::new(5.0) / F::new(3.0) * t875 * t661 + F::new(5.0) / F::new(3.0) * t304 * t665 + F::new(10.0) / F::new(3.0) * t880 * t665 + F::new(10.0) / F::new(3.0) * t308 * t885 * t101 - F::new(0.17058312527037532642e0) * t316 * t891 + F::new(0.80027407411602181738e-1) * t896 * t903 + F::new(0.7107630219598971934e-1) * t908 * t914 + F::new(0.7107630219598971934e-1) * t919 * t914 - F::new(0.17058312527037532642e0) * t333 * t315 * t890 * t324 + F::new(0.80027407411602181738e-1) * t333 * t895 * t899 * t902;
    (t919, t930)
}
