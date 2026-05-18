//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1072/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1072<F: Float>(t137: F, t336: F, t4876: F, t578: F, t2068: F, t4680: F, t8911: F, t1181: F, t23688: F, t599: F, t7346: F, t7433: F, t8966: F) -> (F, F, F, F) {
    let t35080 = t578 * t336 * t4876 * t137;
    let t35084 = t2068 * t4680 * t8911;
    let t35088 = t7346 * t1181 * t599 * t23688;
    let t35090 = t7433 * t8966;
    (t35080, t35084, t35088, t35090)
}
