//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1016/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1016<F: Float>(t2344: F, t2360: F, t33426: F, t3886: F, t2347: F, t9567: F, t1127: F, t1109: F, t17: F, t171: F, t27733: F, t33365: F) -> (F, F, F, F, F) {
    let t150331 = t33426 * t2344 * t2360 * t3886;
    let t150336 = t33426 * t9567 * t2347 * t3886;
    let t150344 = t2344 * t1127;
    let t150351 = t1109 * t171 * t17;
    let t150355 = t27733 * t33365;
    (t150331, t150336, t150344, t150351, t150355)
}
