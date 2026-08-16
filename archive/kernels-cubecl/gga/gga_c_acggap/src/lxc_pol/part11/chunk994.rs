//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 994/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk994<F: Float>(t33682: F, t7887: F, t7990: F, t9076: F, t1620: F, t7973: F, t2331: F, t323: F, t851: F, t2137: F, t32123: F, t1619: F, t322: F) -> (F, F, F, F, F, F) {
    let t33683 = t33682 * t7887;
    let t33686 = F::cast_from(0.34694512752820797848e1_f64) * t7990 * t9076;
    let t33691 = F::cast_from(0.26341796731742046394e1_f64) * t7973 * t1620;
    let t33695 = t851 * t2331 * t323;
    let t33698 = t2137 * t32123;
    let t33699 = t1619 * t322;
    (t33683, t33686, t33691, t33695, t33698, t33699)
}
