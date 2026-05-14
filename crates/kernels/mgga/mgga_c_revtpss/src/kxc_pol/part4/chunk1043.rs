//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1043/1274 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk1043<F: Float>(t13773: F, t13814: F, t13860: F, t13931: F, t13965: F, t14002: F, t14033: F, t14063: F, t225: F, t5774: F, t72: F, t686: F, t3915: F, t5711: F, t786: F, t1364: F) -> (F, F, F, F) {
    let t14066 = t13773 + t13814 + t13860 + t13931 + t13965 + t14002 + t14033 + t14063;
    let t14067 = t14066 * t225;
    let t14078 = t5774 * t72;
    let t14079 = t14078 * t686;
    let t14081 = 0.19514881078765566038e-1 * t3915 * t14079;
    let t14082 = t786 * t5711;
    let t14084 = 0.19514881078765566038e-1 * t14082 * t1364;
    (t14066, t14067, t14081, t14084)
}
