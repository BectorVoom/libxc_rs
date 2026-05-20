//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2931/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2931<F: Float>(t41361: F, t51978: F, t52573: F, t63320: F, t77515: F, t77518: F, t77521: F, t77527: F, t77531: F, t77535: F, t77736: F, t77739: F) -> F {
    let t77923 = F::new(0.61977e1) * t77515 - F::cast_from(0.17215833333333333333e1_f64) * t77518 - F::new(0.929655e1) * t77521 + F::new(0.41678e0) * t77736 - F::new(0.187551e1) * t77739 - t52573 + F::cast_from(0.16068111111111111111e1_f64) * t51978 + F::new(0.20839e0) * t63320 + F::cast_from(0.5356037037037037037e0_f64) * t41361 - F::new(0.103295e1) * t77527 - F::new(0.103295e1) * t77531 + F::new(0.123954e2) * t77535;
    t77923
}
