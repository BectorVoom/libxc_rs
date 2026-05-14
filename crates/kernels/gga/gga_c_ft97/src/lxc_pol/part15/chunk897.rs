//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 897/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk897<F: Float>(t20395: F, t3238: F, t11556: F, t1901: F, t1909: F, t20218: F, t20471: F, t2983: F, t4417: F, t446: F, t452: F, t4589: F, t60756: F, t74959: F, t75584: F, t75586: F, t75588: F, t75590: F, t75624: F, t75642: F, t8424: F, t925: F, t942: F) -> (F, F) {
    let t85928 = t3238 * t20395;
    let t85988 = -4.0 / 3.0 * t446 * t452 * t20471 * t942 - 4.0 / 9.0 * t75584 - 8.0 / 9.0 * t75586 + 8.0 / 27.0 * t75588 + 8.0 / 9.0 * t75590 - 4.0 / 3.0 * t1901 * t1909 * t8424 * t4417 * t4589 + 4.0 / 9.0 * t1901 * t1909 * t74959 * t925 + 8.0 / 9.0 * t75624 + 4.0 / 3.0 * t75642 + 8.0 / 9.0 * t1901 * t11556 * t2983 * t20218 + 16.0 / 27.0 * t60756;
    (t85928, t85988)
}
