//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1085/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1085(t1882: f64, t35689: f64, t35625: f64, t8392: f64, t35641: f64, t35657: f64, t14163: f64, t14200: f64, t142412: f64, t142423: f64, t150045: f64, t150206: f64, t1901: f64, t33692: f64, t33696: f64, t35599: f64, t35604: f64, t42339: f64, t446: f64, t53927: f64, t6061: f64, t65408: f64, t67847: f64, t684: f64, t6947: f64, t729: f64) -> (f64, f64, f64) {
    let t152422 = t1882 * t35689;
    let t152424 = t8392 * t35625;
    let t152450 = t1882 * t35641;
    let t152459 = t1882 * t35657;
    let t152461 = 2.0_f64 / 27.0_f64 * t1901 * t14200 * t150206 + 2.0_f64 / 9.0_f64 * t1901 * t42339 * t35599 * t684 + 2.0_f64 / 3.0_f64 * t1901 * t53927 * t35604 * t684 - 2.0_f64 / 9.0_f64 * t142412 - 4.0_f64 / 9.0_f64 * t1901 * t14163 * t150045 - 2.0_f64 / 3.0_f64 * t446 * t729 * t6947 * t6061 - 2.0_f64 / 9.0_f64 * t152450 - 4.0_f64 / 3.0_f64 * t1901 * t65408 * t33692 - 4.0_f64 / 3.0_f64 * t1901 * t67847 * t33696 - 4.0_f64 / 9.0_f64 * t142423 + 2.0_f64 / 3.0_f64 * t152459;
    (t152422, t152424, t152461)
}
