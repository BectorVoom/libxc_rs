//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 987/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk987<F: Float>(t167: F, t2185: F, t30244: F, t30239: F, t23470: F, t4828: F, t23443: F, t4823: F, t1391: F, t4458: F, t569: F, t6725: F, t925: F, t144: F, t30134: F, t1901: F, t24003: F, t24054: F, t27310: F, t27324: F, t27398: F, t30508: F, t30512: F, t30515: F, t30520: F, t30524: F, t30528: F, t446: F) -> (F, F, F, F, F, F, F, F) {
    let t30532 = t2185 * t167 * t30244;
    let t30536 = t2185 * t167 * t30239;
    let t30540 = t23470 * t4828;
    let t30543 = t23443 * t4823;
    let t30548 = t569 * t1391 * t4458;
    let t30552 = t569 * t6725 * t925;
    let t30555 = t144 * t30134;
    let t30558 = 2.0 / 9.0 * t27310 - t446 * t30508 / 9.0 - 2.0 / 27.0 * t446 * t30512 - 2.0 * t446 * t30515 - 2.0 / 3.0 * t446 * t30520 - 2.0 / 3.0 * t446 * t30524 - t24003 + 4.0 / 3.0 * t446 * t30528 + 4.0 / 3.0 * t446 * t30532 + 2.0 / 3.0 * t446 * t30536 + 2.0 / 9.0 * t27324 + 2.0 / 9.0 * t1901 * t30540 + 2.0 / 9.0 * t1901 * t30543 - 2.0 / 9.0 * t27398 + t24054 + 2.0 / 9.0 * t446 * t30548 - 2.0 / 9.0 * t446 * t30552 + 2.0 / 3.0 * t446 * t30555;
    (t30532, t30536, t30540, t30543, t30548, t30552, t30555, t30558)
}
