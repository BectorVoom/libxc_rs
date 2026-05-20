//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1787/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1787<F: Float>(t1222: F, t1261: F, t12855: F, t13100: F, t17475: F, t21040: F, t24228: F, t24535: F, t247: F, t24792: F, t3604: F, t3625: F, t3626: F, t3720: F, t44225: F, t5312: F, t5381: F, t83392: F, t83394: F, t83435: F, t89822: F, t89826: F, t89863: F, t90042: F, t90262: F, t91012: F) -> F {
    let t91173 = t1222 * t5312 * t89826 / F::new(6.0) - F::new(7.0) / F::new(108.0) * t1222 * t17475 * t89822 - F::cast_from(0.22866142996303859718e-2_f64) * t83392 - F::cast_from(0.2540682555144873302e-2_f64) * t5381 * t24535 - F::cast_from(0.76220476654346199062e-2_f64) * t1261 * t247 * t13100 * t89863 - F::cast_from(0.22866142996303859718e-2_f64) * t83394 - F::cast_from(0.25724410870841842184e-2_f64) * t12855 * t3720 * t90042 * t3604 - F::cast_from(0.11433071498151929859e-2_f64) * t83435 - F::cast_from(0.85748036236139473944e-3_f64) * t3625 * t3626 * t21040 * t91012 - F::cast_from(0.17149607247227894789e-2_f64) * t3625 * t3626 * t21040 * t90262 - F::cast_from(0.2540682555144873302e-2_f64) * t3625 * t44225 * t24228 * t24792;
    t91173
}
