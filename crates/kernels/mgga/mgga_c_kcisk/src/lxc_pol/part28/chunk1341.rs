//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1341/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1341<F: Float>(t34465: F, t9733: F, t33276: F, t9991: F, t34462: F, t9736: F, t34457: F, t33297: F, t34389: F, t33208: F, t17163: F, t34394: F, t9740: F, t34415: F, t9724: F, t10013: F, t12261: F, t2804: F) -> (F, F, F, F, F, F, F, F, F) {
    let t118210 = 0.34722222222222222222e-2 * t9733 * t34465;
    let t118212 = t9991 * t33276;
    let t118223 = t34462 * t9736;
    let t118237 = 0.34722222222222222222e-2 * t34457 * t9736;
    let t118246 = 0.11574074074074074074e-2 * t33297 * t34389;
    let t118248 = 0.11574074074074074074e-2 * t33208 * t34389;
    let t118250 = t9740 * t17163 * t34394;
    let t118275 = t9724 * t34415;
    let t118316 = t2804 * t12261 * t10013;
    (t118210, t118212, t118223, t118237, t118246, t118248, t118250, t118275, t118316)
}
