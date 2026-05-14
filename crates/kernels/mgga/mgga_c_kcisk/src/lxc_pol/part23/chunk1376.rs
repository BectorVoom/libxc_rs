//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1376/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1376<F: Float>(t13955: F, t9815: F, t20160: F, t33388: F, t9426: F, t18971: F, t32045: F, t3759: F, t32102: F, t1286: F, t1411: F, t51845: F, t109504: F, t1339: F, t9814: F, t110281: F, t110655: F, t113629: F, t113695: F, t114231: F, t1220: F, t1299: F, t20: F, t2714: F, t2718: F, t32013: F, t33389: F, t52891: F, t53303: F, t6147: F, t6204: F, t9446: F, t9449: F, t9796: F) -> (F, F, F, F, F, F) {
    let t114302 = t13955 * t9815;
    let t114304 = t20160 * t33388;
    let t114305 = t9426 * t114304;
    let t114308 = t3759 * t32045 * t18971;
    let t114315 = 0.15520416666666666667e-2 * t32102 * t114304;
    let t114334 = t1411 * t32045 * t51845 * t1286;
    let t114337 = t1339 * t109504 * t9814;
    let t114339 = -0.21444444444444444446e-1 * t110281 * t9796 - 0.3684876543209876543e-3 * t114302 - 0.80416666666666666667e-2 * t114305 - 0.27636574074074074073e-2 * t114308 - 0.69444444444444444446e-2 * t114231 * t9449 + 0.12416333333333333334e-1 * t110655 * t33389 - t114315 + 0.13968375e-1 * t32102 * t113629 - 0.20833333333333333334e-1 * t9446 * t6204 * t32013 * t53303 - 0.10416666666666666667e-1 * t9446 * t113695 - 0.10416666666666666667e-1 * t52891 * t2714 * t2718 + 0.55555555555555555558e-1 * t1220 * t6147 * t1299 * t20 * t2718 - 0.33163888888888888888e-2 * t114334 - 0.88437037037037037034e-2 * t114337;
    (t114302, t114304, t114308, t114334, t114337, t114339)
}
