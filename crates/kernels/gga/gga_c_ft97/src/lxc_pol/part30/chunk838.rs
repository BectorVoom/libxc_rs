//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 838/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk838<F: Float>(t141369: F, t33296: F, t24543: F, t33321: F, t24237: F, t33496: F, t1403: F, t2399: F, t7490: F, t33247: F, t681: F, t263: F, t33452: F, t33537: F, t33571: F, t5996: F) -> (F, F, F, F, F, F, F, F) {
    let t141370 = t141369 * t33296;
    let t141384 = t24543 * t33321;
    let t141406 = t24237 * t33496;
    let t141410 = 4.0 / 27.0 * t1403 * t2399 * t7490;
    let t141420 = t1403 * t681 * t33247;
    let t141422 = t33452 * t263;
    let t141431 = t24237 * t33537;
    let t141435 = t5996 * t33571;
    (t141370, t141384, t141406, t141410, t141420, t141422, t141431, t141435)
}
