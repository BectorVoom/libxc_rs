//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1136/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1136<F: Float>(t22583: F, t29478: F, t92466: F, t379: F, t4449: F, t16150: F, t5570: F, t8088: F, t115586: F, t1742: F, t22515: F, t100634: F, t16169: F, t101360: F, t101387: F, t11121: F, t115320: F, t115324: F, t115381: F, t15632: F, t22513: F, t22597: F, t22602: F, t22603: F, t22738: F, t25708: F, t29479: F, t29502: F, t5540: F, t5546: F, t58580: F, t92278: F, t92463: F, t92476: F, t92616: F, t92897: F, t92899: F) -> (F, F, F, F) {
    let t116015 = t22583 * t92466 * t29478;
    let t116017 = t4449 * t379;
    let t116025 = t5570 * t8088 * t16150;
    let t116029 = t22515 * t1742 * t115586;
    let t116033 = t100634 * t1742 * t16169;
    let t116036 = -0.51690243689028715488e-4 * t22603 * t5540 * t115320 - 0.51690243689028715488e-4 * t22603 * t5540 * t115324 + 0.12020514968855939808e-5 * t11121 * t22602 * t5546 * t58580 - 0.12255510004984495842e-5 * t92278 * t22738 * t29502 - 0.10338048737805743098e-4 * t22597 * t5540 * t115381 - 0.85124811172839506172e-2 * t101360 + 0.32054706583615839487e-5 * t15632 * t92616 - 0.34526011664076264185e-5 * t101387 - 0.39591381038172075259e-3 * t92463 * t29479 + 0.49489226297715094073e-4 * t116015 - 0.4945510644553639738e-5 * t92897 * t92899 * t116017 - 0.14846767889314528222e-4 * t22583 * t92476 * t116017 - 0.38306165027777777778e-1 * t25708 * t116025 + 0.30274029503828221194e-3 * t22513 * t116029 + 0.51074886703703703704e-1 * t25708 * t116033;
    (t116025, t116029, t116033, t116036)
}
