//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 953/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk953<F: Float>(t1937: F, t34446: F, t7586: F, t7735: F, t1936: F, t29427: F, t7741: F, t1518: F, t32825: F, t33633: F, t33635: F, t33637: F, t33640: F, t33642: F, t33644: F, t33646: F, t34419: F, t8564: F) -> (F, F, F) {
    let t34447 = t34446 * t1937;
    let t34449 = t7586 * t7735;
    let t34453 = t29427 * t1936;
    let t34455 = t34446 * t1936;
    let t34457 = t7586 * t7741;
    let t34462 = 2.0 * t1518 * t32825 + 2.0 * t33633 + 2.0 * t33635 + 2.0 * t33637 + t33640 + t33642 + t33644 + t33646 + t34419 + 2.0 * t34453 + 2.0 * t34455 + 2.0 * t34457 + t8564;
    (t34447, t34449, t34462)
}
