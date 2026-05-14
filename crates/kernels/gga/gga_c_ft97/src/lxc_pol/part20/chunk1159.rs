//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1159/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1159<F: Float>(t1882: F, t28248: F, t6871: F, t8232: F, t10007: F, t107866: F, t14086: F, t14127: F, t14133: F, t14226: F, t1901: F, t2413: F, t242: F, t2469: F, t28140: F, t28208: F, t28255: F, t28300: F, t28368: F, t28387: F, t446: F, t51853: F, t51901: F, t53513: F, t6135: F, t6161: F, t6861: F, t729: F, t97841: F, t97843: F, t97861: F, t97870: F) -> (F,) {
    let t110889 = 2.0 / 9.0 * t1882 * t28248;
    let t110890 = t8232 * t6871;
    let t110928 = -t110889 - 4.0 / 81.0 * t110890 + 4.0 / 3.0 * t446 * t242 * t107866 + 2.0 / 3.0 * t97841 + 2.0 / 9.0 * t97843 - 4.0 / 3.0 * t1901 * t51853 * t28368 + 2.0 / 3.0 * t446 * t729 * t2469 * t28255 - 4.0 / 9.0 * t1901 * t51901 * t28208 - 4.0 / 9.0 * t1901 * t53513 * t28387 + 2.0 * t1901 * t14127 * t28300 * t14226 + 2.0 * t1901 * t28140 * t6161 * t14133 - t1901 * t10007 * t6135 * t14086 / 9.0 - 2.0 / 27.0 * t97861 - t1901 * t10007 * t6861 * t2413 / 9.0 - 8.0 / 27.0 * t97870;
    (t110928,)
}
