//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 734/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk734<F: Float>(t7679: F, t824: F, t840: F, t871: F, t875: F, t2843: F, t296: F, t2749: F, t7629: F, t681: F, t7664: F, t89: F, t34013: F, t7686: F, t1901: F, t193: F, t34199: F, t34204: F, t34209: F, t34213: F, t34217: F, t34221: F, t446: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t34225 = t7679 * t824;
    let t34227 = t840 * t871 * t34225;
    let t34230 = t7679 * t875;
    let t34231 = t2843 * t34230;
    let t34232 = t296 * t34231;
    let t34236 = t840 * t2749 * t7629;
    let t34241 = t89 * t681 * t7664 / 9.0;
    let t34242 = t296 * t34013;
    let t34246 = t840 * t7686 * t824;
    let t34249 = -2.0 / 9.0 * t1901 * t34199 + t1901 * t34204 / 9.0 + t1901 * t34209 / 9.0 - 2.0 / 3.0 * t446 * t34213 - 2.0 / 3.0 * t446 * t34217 + t89 * t193 * t34221 / 3.0 + t446 * t34227 / 3.0 + 2.0 / 3.0 * t446 * t34232 + 2.0 / 3.0 * t446 * t34236 - t34241 - t446 * t34242 / 3.0 - t446 * t34246 / 3.0;
    (t34225, t34227, t34230, t34231, t34232, t34236, t34241, t34242, t34246, t34249)
}
