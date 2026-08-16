//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1687/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1687<F: Float>(t26895: F, t26982: F, t27183: F, t27238: F, t3: F, t112: F, t7945: F, t1458: F, t7056: F, t2039: F, t4072: F, t671: F, t7801: F) -> (F, F, F, F, F, F) {
    let t27240 = t26895 + t26982 + t27183 + t27238;
    let t27241 = t3 * t27240;
    let t27254 = t7945 * t112;
    let t27273 = t7056 * t1458;
    let t27276 = t2039 * t4072;
    let t27281 = t7801 * t671;
    (t27240, t27241, t27254, t27273, t27276, t27281)
}
