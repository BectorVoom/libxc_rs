//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 967/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk967<F: Float>(t34265: F, t6210: F, t1466: F, t34325: F, t681: F, t34321: F, t1506: F, t668: F, t33983: F, t683: F, t317: F, t33953: F) -> (F, F, F, F, F, F) {
    let t142918 = t6210 * t34265;
    let t142925 = t1466 * t681 * t34325;
    let t142935 = t1466 * t681 * t34321;
    let t142941 = t1506 * t668;
    let t142946 = t683 * t33983;
    let t142950 = t33953 * t317;
    (t142918, t142925, t142935, t142941, t142946, t142950)
}
