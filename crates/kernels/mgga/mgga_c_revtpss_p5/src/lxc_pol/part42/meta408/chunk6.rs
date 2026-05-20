//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1435/1505 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1435<F: Float>(t13784: F, t13790: F, t13789: F, t13880: F, t13943: F, t13949: F, t13954: F, t13956: F, t5671: F, t9776: F, t9780: F, t9786: F, t9791: F, t9796: F, t9799: F) -> F {
    let t22145 = t13790 * t13784;
    let t22146 = t13789 * t22145;
    let t22153 = -F::cast_from(0.76220476654346199061e-4_f64) * t9776 - F::cast_from(0.22675591804667994221e-1_f64) * t9780 + t13880 - F::cast_from(0.34299214494455789578e-2_f64) * t5671 * t22146 - t9786 - t9791 - F::cast_from(0.45178982497454656791e-5_f64) * t9796 - F::cast_from(0.18071592998981862716e-4_f64) * t9799 + t13943 - F::cast_from(0.60976381323476959249e-3_f64) * t13949 + t13954 + F::cast_from(0.50820002809285328224e-5_f64) * t13956;
    t22153
}
