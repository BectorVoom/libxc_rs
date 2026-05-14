//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1281/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1281<F: Float>(t1308: F, t388: F, t41167: F, t110284: F, t9426: F, t1220: F, t4153: F, t32164: F, t9442: F, t32105: F, t9439: F, t18681: F, t2715: F, t2717: F, t55867: F, t9445: F) -> (F, F, F, F, F, F, F) {
    let t110474 = t41167 * t388 * t1308;
    let t110477 = t9426 * t110284;
    let t110492 = t1220 * t4153 * t1308;
    let t110503 = t32164 * t9442;
    let t110505 = t9439 * t32105;
    let t110509 = 0.38580246913580246915e-2 * t2715 * t18681 * t2717;
    let t110524 = t9445 * t55867;
    (t110474, t110477, t110492, t110503, t110505, t110509, t110524)
}
