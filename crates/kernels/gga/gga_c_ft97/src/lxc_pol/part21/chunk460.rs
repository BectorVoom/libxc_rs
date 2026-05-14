//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 460/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk460<F: Float>(t379: F, t5717: F, t1909: F, t1332: F, t432: F, t452: F, t488: F, t1825: F, t83: F, t492: F, t1852: F, t5672: F, t5689: F, t5669: F, t5678: F, t5682: F, t5686: F, t5694: F, t5698: F, t5702: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t5718 = t5717 * t379;
    let t5719 = t1909 * t5718;
    let t5722 = t1332 * t432;
    let t5724 = t452 * t488 * t5722;
    let t5727 = t1825 * t1332;
    let t5728 = t83 * t5727;
    let t5731 = t1332 * t492;
    let t5732 = t1852 * t5731;
    let t5733 = t83 * t5732;
    let t5737 = t5672 / 6.0;
    let t5740 = t5689 / 3.0;
    let t5743 = t5669 / 4.0 + t5737 + t5678 / 6.0 + t5682 - t5686 / 2.0 + t5740 + t5694 / 3.0 + 2.0 * t5698 - t5702;
    (t5718, t5719, t5722, t5724, t5727, t5728, t5731, t5732, t5733, t5737, t5740, t5743)
}
