//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1080/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1080<F: Float>(t10969: F, t32411: F, t34707: F, t8392: F, t1852: F, t3255: F, t7281: F, t1882: F, t34663: F, t6547: F, t91496: F, t11593: F, t11837: F, t137812: F, t137814: F, t144657: F, t1901: F, t1909: F, t3052: F, t3238: F, t32488: F, t32630: F, t34511: F, t432: F, t446: F, t452: F, t488: F, t6534: F, t7229: F, t83: F, t93636: F) -> (F, F, F, F, F) {
    let t146143 = t10969 * t32411;
    let t146150 = t8392 * t34707;
    let t146171 = t1852 * t7281 * t3255;
    let t146175 = t1882 * t34663;
    let t146182 = t91496 * t6547;
    let t146201 = -F::new(2.0) / F::new(3.0) * t446 * t83 * t144657 + F::new(4.0) / F::new(9.0) * t137812 - F::new(2.0) / F::new(27.0) * t137814 + F::new(2.0) / F::new(3.0) * t446 * t83 * t146171 - F::new(2.0) / F::new(9.0) * t146175 + t446 * t452 * t488 * t34511 * t432 / F::new(3.0) + F::new(4.0) / F::new(3.0) * t446 * t83 * t146182 - F::new(4.0) / F::new(9.0) * t11593 * t1909 * t32488 * t3052 + F::new(2.0) / F::new(9.0) * t1901 * t93636 * t6534 + t446 * t452 * t3238 * t32630 / F::new(3.0) + F::new(2.0) / F::new(3.0) * t446 * t452 * t11837 * t7229;
    (t146143, t146150, t146171, t146182, t146201)
}
