//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 989/1093 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk989<F: Float>(t28938: F, t7900: F, t2107: F, t22475: F, t1502: F, t1519: F, t1843: F, t2014: F, t2052: F, t2089: F, t28653: F, t30558: F, t30563: F, t30571: F, t30578: F, t30581: F, t30584: F, t30586: F, t30589: F, t30612: F, t4248: F, t508: F, t569: F, t5877: F, t5884: F, t5921: F, t651: F, t6765: F, t7359: F, t7732: F, t7969: F, t7984: F, t7988: F, t8065: F) -> (F, F, F) {
    let t30614 = t28938 * t7900;
    let t30617 = t2107 * t22475;
    let t30625 = -2.0 * t7969 * t1843 - 2.0 * t651 * t30558 - 4.0 * t7732 * t7984 - 2.0 * t651 * t30563 - 2.0 * t651 * t30571 - 4.0 * t4248 * t7988 - 4.0 * t28653 * t1519 - 4.0 * t651 * t30578 + 3.0 * t2014 * t30581 - t2014 * t30584 + 6.0 * t2014 * t30586 - 2.0 * t30589 * t508 - t5877 * t2089 - 2.0 * t1502 * t8065 + t30612 * t569 + 6.0 * t2014 * t30614 + 2.0 * t2014 * t30617 - t2052 * t6765 - 2.0 * t5884 * t2089 - 2.0 * t7359 * t5921;
    (t30614, t30617, t30625)
}
