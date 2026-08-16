//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 771/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk771<F: Float>(t5257: F, t5317: F, t539: F, t1835: F, t225: F, t1385: F, t1842: F, t3887: F, t3787: F, t68: F, t544: F, t1824: F, t562: F) -> (F, F, F, F, F, F, F) {
    let t5318 = t5257 + t5317;
    let t5319 = t539 * t5318;
    let t5321 = t1835 * t225;
    let t5325 = t1842 * t1385;
    let t5326 = t3887 * t5325;
    let t5333 = t68 * t3787;
    let t5334 = t544 * t5333;
    let t5335 = t562 * t1824;
    (t5318, t5319, t5321, t5326, t5333, t5334, t5335)
}
