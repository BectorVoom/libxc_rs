//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1165/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1165<F: Float>(t26375: F, t531: F, t530: F, t7535: F, t10263: F, t1450: F, t1453: F, t2014: F, t2106: F, t2107: F, t2108: F, t2320: F, t2322: F, t25089: F, t25177: F, t25188: F, t25802: F, t25865: F, t26154: F, t26162: F, t26376: F, t26380: F, t26411: F, t26674: F, t26699: F, t46304: F, t508: F, t649: F, t7235: F, t7238: F, t7315: F, t7359: F, t7474: F, t7488: F, t7489: F, t7536: F, t9400: F, t94349: F, t95002: F, t95019: F, t95371: F) -> F {
    let t95464 = t531 * t26375;
    let t95472 = t530 * t7535;
    let t95499 = -F::cast_from(3.0_f64) * t2320 * t7474 - t2014 * t2107 * t46304 - F::cast_from(3.0_f64) * t2014 * t7536 * t25802 + F::cast_from(6.0_f64) * t2014 * t9400 * t2106 * t1450 + F::cast_from(3.0_f64) * t26699 * t1453 - F::cast_from(6.0_f64) * t2014 * t2107 * t94349 + F::cast_from(9.0_f64) * t2014 * t95464 * t7238 - F::cast_from(3.0_f64) * t649 * t26674 - F::cast_from(6.0_f64) * t95371 * t508 + F::cast_from(18.0_f64) * t2014 * t95472 * t25865 + F::cast_from(9.0_f64) * t25188 * t7489 - F::cast_from(6.0_f64) * t7235 * t26380 + t95019 * t2108 + F::cast_from(6.0_f64) * t2014 * t7536 * t25177 + F::cast_from(3.0_f64) * t2014 * t7488 * t95002 - F::cast_from(6.0_f64) * t2322 * t26154 + F::cast_from(9.0_f64) * t2014 * t26411 * t25089 + F::cast_from(18.0_f64) * t7235 * t26162 - F::cast_from(6.0_f64) * t7359 * t10263 - F::cast_from(3.0_f64) * t2014 * t26376 * t7315;
    t95499
}
