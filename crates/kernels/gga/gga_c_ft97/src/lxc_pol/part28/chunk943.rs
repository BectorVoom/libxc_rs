//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 943/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk943<F: Float>(t1882: F, t34629: F, t102921: F, t5731: F, t34732: F, t34718: F, t34722: F, t22943: F, t25590: F, t34726: F, t8392: F, t102776: F, t103108: F, t103823: F, t11490: F, t117775: F, t11810: F, t11906: F, t1307: F, t137826: F, t137836: F, t1901: F, t23339: F, t26042: F, t26145: F, t26167: F, t32516: F, t3291: F, t34670: F, t446: F, t452: F, t47007: F, t5718: F, t5722: F, t6465: F, t6538: F, t7211: F, t83: F, t91583: F, t92049: F) -> (F, F, F) {
    let t146206 = t1882 * t34629;
    let t146208 = t102921 * t5731;
    let t146212 = t1882 * t34732;
    let t146214 = t1882 * t34718;
    let t146216 = t1882 * t34722;
    let t146218 = t22943 * t25590;
    let t146237 = t8392 * t34726;
    let t146263 = 2.0 / 9.0 * t137826 - t146206 / 9.0 + 4.0 / 3.0 * t446 * t83 * t146208 + 2.0 / 9.0 * t146212 - 2.0 / 9.0 * t146214 + 2.0 / 9.0 * t146216 - t137836 + 4.0 / 3.0 * t446 * t83 * t146218 - t446 * t452 * t3291 * t7211 / 3.0 - 4.0 / 3.0 * t1901 * t11810 * t103108 * t5722 - 4.0 / 3.0 * t1901 * t11490 * t117775 * t5731 + 2.0 / 9.0 * t1901 * t103823 * t5718 + 4.0 / 9.0 * t146237 + 2.0 / 9.0 * t1901 * t92049 * t6465 - 4.0 / 3.0 * t1901 * t102776 * t26167 - 4.0 / 3.0 * t1901 * t47007 * t34670 - 4.0 / 3.0 * t1901 * t11810 * t91583 * t6538 - 4.0 / 3.0 * t1901 * t11810 * t23339 * t26145 + t1901 * t11906 * t32516 / 9.0 - 2.0 / 3.0 * t446 * t452 * t26042 * t1307;
    (t146208, t146218, t146263)
}
