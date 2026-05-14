//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1212/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1212<F: Float>(t33451: F, t9426: F, t3973: F, t9804: F, t9446: F, t5601: F, t9447: F, t6187: F, t1299: F, t20: F, t2158: F, t1220: F, t9442: F, t9801: F, t2718: F, t32019: F, t32022: F, t32096: F, t33384: F, t33460: F, t9429: F, t9454: F, t9805: F, t9809: F) -> (F, F, F, F, F, F) {
    let t33463 = t9426 * t33451;
    let t33469 = t3973 * t9804;
    let t33470 = t9446 * t33469;
    let t33476 = t9447 * t5601;
    let t33477 = t6187 * t33476;
    let t33481 = t2158 * t1299 * t20;
    let t33482 = t1220 * t33481;
    let t33485 = t9801 * t9442;
    let t33487 = 0.10416666666666666667e-1 * t33384 * t9454 + 0.10416666666666666667e-1 * t33384 * t9429 + 0.40208333333333333335e-2 * t33460 * t9429 + 0.13402777777777777778e-2 * t33463 + 0.10416666666666666667e-1 * t32096 * t9809 + 0.10416666666666666667e-1 * t32019 * t9809 - 0.11574074074074074074e-2 * t33470 + 0.92592592592592592595e-2 * t32022 * t9805 - 0.34722222222222222223e-2 * t32096 * t9805 - 0.69444444444444444446e-2 * t9446 * t33477 + 0.27777777777777777779e-1 * t33482 * t2718 - 0.34722222222222222223e-2 * t33485;
    (t33469, t33476, t33477, t33481, t33482, t33487)
}
