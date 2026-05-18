//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1085/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1085<F: Float>(t1882: F, t35689: F, t35625: F, t8392: F, t35641: F, t35657: F, t14163: F, t14200: F, t142412: F, t142423: F, t150045: F, t150206: F, t1901: F, t33692: F, t33696: F, t35599: F, t35604: F, t42339: F, t446: F, t53927: F, t6061: F, t65408: F, t67847: F, t684: F, t6947: F, t729: F) -> (F, F, F) {
    let t152422 = t1882 * t35689;
    let t152424 = t8392 * t35625;
    let t152450 = t1882 * t35641;
    let t152459 = t1882 * t35657;
    let t152461 = F::new(2.0) / F::new(27.0) * t1901 * t14200 * t150206 + F::new(2.0) / F::new(9.0) * t1901 * t42339 * t35599 * t684 + F::new(2.0) / F::new(3.0) * t1901 * t53927 * t35604 * t684 - F::new(2.0) / F::new(9.0) * t142412 - F::new(4.0) / F::new(9.0) * t1901 * t14163 * t150045 - F::new(2.0) / F::new(3.0) * t446 * t729 * t6947 * t6061 - F::new(2.0) / F::new(9.0) * t152450 - F::new(4.0) / F::new(3.0) * t1901 * t65408 * t33692 - F::new(4.0) / F::new(3.0) * t1901 * t67847 * t33696 - F::new(4.0) / F::new(9.0) * t142423 + F::new(2.0) / F::new(3.0) * t152459;
    (t152422, t152424, t152461)
}
