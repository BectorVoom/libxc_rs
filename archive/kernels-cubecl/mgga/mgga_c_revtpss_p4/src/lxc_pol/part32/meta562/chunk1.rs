//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1883/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1883<F: Float>(t92991: F, t14861: F, t25227: F, t2661: F, t1565: F, t93066: F, t25222: F, t4345: F, t4349: F, t93072: F, t14673: F, t92955: F) -> (F, F, F, F, F, F) {
    let t99004 = F::cast_from(0.4065600224742826258e-4_f64) * t92991;
    let t99006 = t2661 * t25227 * t14861;
    let t99009 = t93066 * t1565;
    let t99011 = t25222 * t4345;
    let t99013 = t93072 * t4349;
    let t99019 = t92955 * t14673;
    (t99004, t99006, t99009, t99011, t99013, t99019)
}
