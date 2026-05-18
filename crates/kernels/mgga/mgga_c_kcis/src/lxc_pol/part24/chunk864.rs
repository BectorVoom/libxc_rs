//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 864/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk864<F: Float>(t2960: F, t6338: F, t934: F, t18653: F, t2970: F, t26: F, t18645: F, t18661: F, t18669: F, t18674: F, t18679: F, t18683: F, t18828: F, t18830: F, t18833: F, t18835: F) -> (F, F, F) {
    let t18889 = t2960 * t6338;
    let t18890 = t18889 * t934;
    let t18903 = t2970 * t18653;
    let t18904 = t26 * t18903;
    let t18906 = F::new(0.1898925e1) * t18835 + F::new(0.142419375e1) * t18828 - F::new(0.1898925e1) * t18830 - F::new(0.9494625e0) * t18833 - F::new(0.19931111111111111111e0) * t18674 + F::new(0.59793333333333333334e0) * t18679 + F::new(0.66437037037037037037e-1) * t18645 - F::new(0.19931111111111111111e0) * t18661 + F::new(0.99655555555555555557e-1) * t18669 - F::new(0.29896666666666666667e0) * t18683 + F::new(0.16431333333333333333e0) * t18904;
    (t18890, t18904, t18906)
}
