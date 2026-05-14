//! GGA_C_GAPC lxc pol — lxc_pol part 26 (v4rho2sigma2_5) CSE chunk 1053/1126 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part26_v4rho2sigma2_5_chunk1053<F: Float>(t1388: F, t2983: F, t3663: F, t1030: F, t33619: F, t8716: F, t1971: F, t505: F, t8448: F, t9272: F, t11356: F, t26007: F, t9304: F, t2993: F, t3001: F, t33158: F) -> (F, F, F, F, F, F) {
    let t35048 = t1388 * t3663 * t2983;
    let t35050 = t1030 * t33619;
    let t35051 = t35050 * t8716;
    let t35056 = t1030 * t1971 * t8448 * t505 * t9272;
    let t35059 = t9304 * t11356 * t26007;
    let t35062 = t2993 * t33158 * t3001;
    (t35048, t35050, t35051, t35056, t35059, t35062)
}
