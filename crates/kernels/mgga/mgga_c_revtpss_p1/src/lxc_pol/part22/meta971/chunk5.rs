//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3249/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3249<F: Float>(t2674: F, t40683: F, t61639: F, t18441: F, t9775: F, t4423: F, t231: F, t10698: F, t10770: F, t18469: F, t2430: F, t2745: F, t2754: F, t40425: F, t50409: F, t5966: F, t61623: F, t61628: F, t61630: F, t61632: F, t825: F, t827: F, t828: F, t851: F) -> (F, F, F) {
    let t61641 = t2674 * t40683 * t61639;
    let t61645 = t9775 * t18441;
    let t61647 = t4423 * t4423;
    let t61648 = t61647 * t231;
    let t61657 = -F::cast_from(0.27104001498285508387e-3_f64) * t61623 + F::cast_from(0.11433071498151929859e-3_f64) * t61628 + F::cast_from(0.24009450146119052704e0_f64) * t61630 - F::cast_from(0.80031500487063509015e-1_f64) * t61632 - F::cast_from(0.25724410870841842183e-1_f64) * t851 * t10698 * t828 * t5966 * t2430 - F::cast_from(0.30492001685571196935e-2_f64) * t61641 - F::cast_from(0.25692334753583138158e-2_f64) * t40425 - F::cast_from(0.40015750243531754508e-2_f64) * t50409 + F::cast_from(0.60976381323476959249e-3_f64) * t61645 - F::cast_from(0.42874018118069736972e-3_f64) * t825 * t827 * t828 * t61648 - F::cast_from(0.42874018118069736972e-2_f64) * t2745 * t10770 * t18469 * t2754;
    (t61647, t61648, t61657)
}
