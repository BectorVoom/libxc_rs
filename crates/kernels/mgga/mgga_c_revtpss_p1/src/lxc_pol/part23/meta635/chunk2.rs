//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2335/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2335<F: Float>(t39490: F, t39492: F, t39495: F, t39498: F, t39501: F, t39506: F, t39508: F, t39510: F, t39512: F, t39515: F, t682: F, t701: F) -> F {
    let t39520 = F::new(1.0) * t682 * (-F::cast_from(0.21099166666666666667e1_f64) * t39490 + F::new(0.202552e2) * t39492 - F::cast_from(0.75019259259259259258e1_f64) * t39495 + F::cast_from(0.6564185185185185185e1_f64) * t39498 + F::cast_from(0.31003950617283950618e1_f64) * t39501 + F::cast_from(0.68258333333333333335e-1_f64) * t39506 - F::cast_from(0.10921333333333333333e1_f64) * t39508 + F::cast_from(0.12134814814814814815e1_f64) * t39510 + F::cast_from(0.10617962962962962963e1_f64) * t39512 + F::cast_from(0.13388493827160493828e1_f64) * t39515) * t701;
    t39520
}
