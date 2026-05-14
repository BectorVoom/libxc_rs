//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 577/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk577<F: Float>(t373: F, t7998: F, t1608: F, t1689: F, t1691: F, t1696: F, t1609: F, t77: F, t1593: F, t1615: F, t1630: F, t1619: F, t1681: F, t1711: F, t371: F, t407: F) -> (F, F, F, F, F, F, F) {
    let t7999 = t7998 * t373;
    let t8000 = t1608 * t7999;
    let t8002 = t1689 * t1691;
    let t8003 = t8002 * t1696;
    let t8007 = t77 * t1609;
    let t8008 = t8007 * t1593;
    let t8009 = t1608 * t8008;
    let t8014 = t1615 * t1630;
    let t8015 = t1608 * t8014;
    let t8018 = t1619 * t1681;
    let t8042 = t371 * t1711;
    let t8050 = t407 * t407;
    (t8000, t8003, t8009, t8015, t8018, t8042, t8050)
}
