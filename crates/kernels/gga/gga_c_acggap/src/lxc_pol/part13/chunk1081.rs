//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1081/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1081<F: Float>(t3073: F, t31056: F, t33953: F, t4241: F, t13364: F, t13299: F, t2001: F, t4344: F, t30769: F, t4349: F, t7741: F, t30773: F) -> (F, F, F, F, F, F) {
    let t34833 = t3073 * t31056;
    let t34834 = t33953 * t4241;
    let t34836 = t34833 * t13364 * t34834;
    let t34837 = F::new(0.42874018118069736972e-3) * t34836;
    let t34839 = t34833 * t13299 * t34834;
    let t34840 = F::new(0.62896184579208304136e-3) * t34839;
    let t34841 = t2001 * t4344;
    let t34843 = F::new(0.68598428988911579156e-2) * t30769;
    let t34844 = t7741 * t4349;
    let t34846 = F::new(0.42874018118069736972e-3) * t30773;
    (t34837, t34840, t34841, t34843, t34844, t34846)
}
