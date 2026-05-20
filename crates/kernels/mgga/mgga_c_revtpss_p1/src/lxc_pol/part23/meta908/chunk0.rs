//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2916/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2916<F: Float>(t23535: F, t2880: F, t918: F, t51914: F, t51915: F, t63240: F, t63242: F, t77663: F, t77667: F, t77670: F, t77672: F, t77674: F, t77676: F) -> (F, F) {
    let t77679 = t2880 * t23535 * t918;
    let t77681 = t51914 - F::cast_from(0.91983333333333333333e-1_f64) * t51915 - F::new(0.11038e0) * t77663 + F::new(0.99342e0) * t63240 - F::new(0.66228e0) * t63242 + F::cast_from(0.24528888888888888889e-1_f64) * t77667 - F::new(0.82785e-1) * t77670 - F::cast_from(0.1237865625e0_f64) * t77672 + F::cast_from(0.247573125e0_f64) * t77674 + F::cast_from(0.247573125e0_f64) * t77676 - F::new(0.1294625e1) * t77679;
    (t77679, t77681)
}
