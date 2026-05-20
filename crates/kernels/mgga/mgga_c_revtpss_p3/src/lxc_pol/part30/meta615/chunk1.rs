//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2121/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2121<F: Float>(t1907: F, t3829: F, t28167: F, t8717: F, t25082: F, t28197: F, t73488: F, t13625: F, t33651: F, t25090: F, t7898: F, t28187: F, t7235: F) -> (F, F, F, F, F) {
    let t98519 = t1907 * t3829;
    let t98522 = F::new(6.0) * t28167 * t8717 * t98519;
    let t98525 = F::new(6.0) * t25082 * t28197 * t73488;
    let t98528 = F::new(6.0) * t25082 * t33651 * t13625;
    let t98530 = F::new(3.0) * t7898 * t25090;
    let t98532 = F::new(2.0) * t7235 * t28187;
    (t98522, t98525, t98528, t98530, t98532)
}
