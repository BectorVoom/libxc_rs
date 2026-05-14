//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1202/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1202<F: Float>(t258: F, t30859: F, t109652: F, t3864: F, t2568: F, t3972: F, t6940: F, t1168: F, t27889: F, t1882: F, t31102: F, t110498: F, t110503: F, t110517: F, t110612: F, t14127: F, t14175: F, t18211: F, t18216: F, t1901: F, t242: F, t2469: F, t2599: F, t28108: F, t28299: F, t28300: F, t30930: F, t31114: F, t31234: F, t3977: F, t446: F, t5053: F, t6187: F, t684: F, t729: F, t762: F, t9787: F) -> (F, F, F, F) {
    let t122409 = t258 * t30859;
    let t122422 = t109652 * t3864;
    let t122427 = t2568 * t6940 * t3972;
    let t122432 = t2568 * t27889 * t1168;
    let t122444 = t1882 * t31102;
    let t122458 = -t110498 + t1901 * t2599 * t122409 * t684 / 9.0 + 2.0 * t1901 * t14127 * t28300 * t18211 + 8.0 * t1901 * t28299 * t110612 * t18216 - t110503 + 4.0 / 3.0 * t446 * t242 * t122422 + 4.0 / 3.0 * t446 * t242 * t122427 + t110517 + 4.0 / 3.0 * t446 * t242 * t122432 - 2.0 / 9.0 * t1901 * t14175 * t30930 * t684 + 2.0 / 3.0 * t446 * t729 * t3977 * t28108 + t122444 / 9.0 + t446 * t729 * t2469 * t31234 / 3.0 + t446 * t729 * t762 * t6187 * t5053 / 3.0 - 2.0 / 9.0 * t1901 * t9787 * t31114;
    (t122422, t122427, t122432, t122458)
}
