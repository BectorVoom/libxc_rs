//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1030/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1030<F: Float>(t26375: F, t531: F, t530: F, t7535: F, t10263: F, t1450: F, t1453: F, t2014: F, t2106: F, t2107: F, t2108: F, t2320: F, t2322: F, t25089: F, t25177: F, t25188: F, t25802: F, t25865: F, t26154: F, t26162: F, t26376: F, t26380: F, t26411: F, t26674: F, t26699: F, t46304: F, t508: F, t649: F, t7235: F, t7238: F, t7315: F, t7359: F, t7474: F, t7488: F, t7489: F, t7536: F, t9400: F, t94349: F, t95002: F, t95019: F, t95371: F) -> (F,) {
    let t95464 = t531 * t26375;
    let t95472 = t530 * t7535;
    let t95499 = -3.0 * t2320 * t7474 - t2014 * t2107 * t46304 - 3.0 * t2014 * t7536 * t25802 + 6.0 * t2014 * t9400 * t2106 * t1450 + 3.0 * t26699 * t1453 - 6.0 * t2014 * t2107 * t94349 + 9.0 * t2014 * t95464 * t7238 - 3.0 * t649 * t26674 - 6.0 * t95371 * t508 + 18.0 * t2014 * t95472 * t25865 + 9.0 * t25188 * t7489 - 6.0 * t7235 * t26380 + t95019 * t2108 + 6.0 * t2014 * t7536 * t25177 + 3.0 * t2014 * t7488 * t95002 - 6.0 * t2322 * t26154 + 9.0 * t2014 * t26411 * t25089 + 18.0 * t7235 * t26162 - 6.0 * t7359 * t10263 - 3.0 * t2014 * t26376 * t7315;
    (t95499,)
}
