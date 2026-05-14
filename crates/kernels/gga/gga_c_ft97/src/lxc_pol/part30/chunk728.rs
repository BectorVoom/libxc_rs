//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 728/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk728<F: Float>(t684: F, t7686: F, t835: F, t1476: F, t6386: F, t840: F, t871: F, t1882: F, t7631: F, t1901: F, t34062: F, t34067: F, t34070: F, t34074: F, t34078: F, t34083: F, t34086: F, t34091: F, t34095: F, t446: F) -> (F, F, F, F, F) {
    let t34099 = t835 * t7686 * t684;
    let t34102 = t1476 * t6386;
    let t34104 = t840 * t871 * t34102;
    let t34108 = 2.0 / 9.0 * t1882 * t7631;
    let t34109 = 2.0 / 3.0 * t446 * t34062 + t446 * t34067 / 3.0 + 2.0 / 9.0 * t1901 * t34070 - 4.0 / 3.0 * t1901 * t34074 - 4.0 / 3.0 * t1901 * t34078 - 2.0 / 9.0 * t1901 * t34083 + 2.0 / 9.0 * t1901 * t34086 + 4.0 / 3.0 * t446 * t34091 + 4.0 / 3.0 * t446 * t34095 - t446 * t34099 / 9.0 + 2.0 / 3.0 * t446 * t34104 - t34108;
    (t34099, t34102, t34104, t34108, t34109)
}
