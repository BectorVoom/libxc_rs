//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 705/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk705<F: Float>(t32417: F, t83: F, t1825: F, t452: F, t7229: F, t1307: F, t5743: F, t488: F, t492: F, t7211: F, t432: F, t7281: F, t379: F, t8557: F, t1901: F, t32591: F, t32594: F, t32599: F, t32603: F, t32607: F, t32610: F, t446: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t32613 = t83 * t32417;
    let t32617 = t452 * t1825 * t7229;
    let t32620 = t1307 * t5743;
    let t32622 = t452 * t488 * t32620;
    let t32625 = t7211 * t492;
    let t32627 = t452 * t488 * t32625;
    let t32630 = t7281 * t432;
    let t32632 = t452 * t488 * t32630;
    let t32635 = t7229 * t379;
    let t32636 = t8557 * t32635;
    let t32639 = 2.0 / 3.0 * t446 * t32591 + 2.0 / 9.0 * t1901 * t32594 + t1901 * t32599 / 9.0 - 4.0 / 3.0 * t1901 * t32603 - 4.0 / 3.0 * t1901 * t32607 + 2.0 / 9.0 * t1901 * t32610 + 4.0 / 3.0 * t446 * t32613 + 2.0 / 3.0 * t446 * t32617 + 2.0 / 3.0 * t446 * t32622 + t446 * t32627 / 3.0 + t446 * t32632 / 3.0 - 2.0 / 9.0 * t1901 * t32636;
    (t32613, t32617, t32620, t32622, t32625, t32627, t32630, t32632, t32635, t32636, t32639)
}
