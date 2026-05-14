//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 717/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk717<F: Float>(t1168: F, t5147: F, t2568: F, t242: F, t1091: F, t5073: F, t10007: F, t18685: F, t10079: F, t21362: F, t265: F, t724: F, t21355: F, t2594: F, t13872: F, t18188: F, t18190: F, t18427: F, t1901: F, t21474: F, t21479: F, t21483: F, t21488: F, t21492: F, t21496: F, t446: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t21499 = t1168 * t5147;
    let t21500 = t2568 * t21499;
    let t21501 = t242 * t21500;
    let t21504 = t5073 * t1091;
    let t21505 = t10007 * t21504;
    let t21509 = t18685 * t1091;
    let t21510 = t10079 * t21509;
    let t21515 = t724 * t265 * t21362;
    let t21519 = t2594 * t265 * t21355;
    let t21522 = t18188 / 3.0 + 2.0 / 3.0 * t18190 - 2.0 * t446 * t21474 + 2.0 * t446 * t21479 + 2.0 * t446 * t21483 + t446 * t21488 + t446 * t21492 - 2.0 * t446 * t21496 + 2.0 * t446 * t21501 - 2.0 / 3.0 * t1901 * t21505 - 4.0 / 9.0 * t13872 - 2.0 / 3.0 * t1901 * t21510 - 2.0 / 9.0 * t18427 - 2.0 / 3.0 * t446 * t21515 + 4.0 / 9.0 * t446 * t21519;
    (t21499, t21500, t21501, t21504, t21505, t21509, t21510, t21515, t21519, t21522)
}
