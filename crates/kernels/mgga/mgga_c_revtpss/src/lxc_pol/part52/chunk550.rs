//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 550/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk550<F: Float>(t4631: F, t935: F, t915: F, t1609: F, t2926: F, t934: F, t2924: F, t2848: F, t2930: F, t4571: F, t4576: F, t4581: F, t4585: F) -> (F, F, F) {
    let t4632 = t4631 * t935;
    let t4634 = F::new(1.0) * t915 * t4632;
    let t4635 = t1609 * t2926;
    let t4636 = t4635 * t934;
    let t4638 = F::cast_from(0.16081979498692535067e2_f64) * t2924 * t4636;
    let t4644 = t2930 + F::cast_from(0.57077777777777777777e-2_f64) * t2848 + F::cast_from(0.57077777777777777777e-2_f64) * t4571 - F::cast_from(0.11415555555555555555e-1_f64) * t4576 + F::cast_from(0.34246666666666666666e-1_f64) * t4581 - F::cast_from(0.17123333333333333333e-1_f64) * t4585;
    (t4634, t4638, t4644)
}
