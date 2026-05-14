//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1211/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1211<F: Float>(t112441: F, t112773: F, t112775: F, t112777: F, t112778: F, t112784: F, t112785: F, t112790: F, t112795: F, t112803: F, t112807: F, t15191: F, t15229: F, t15395: F, t15460: F, t1901: F, t24909: F, t24913: F, t2867: F, t2877: F, t296: F, t446: F, t7105: F, t72190: F, t98809: F) -> (F,) {
    let t112811 = 8.0 / 3.0 * t1901 * t72190 * t7105 * t2867 + t112773 + t112775 + t112777 - 4.0 / 27.0 * t112778 - t446 * t296 * t112441 / 3.0 + t112784 - 4.0 / 3.0 * t1901 * t15460 * t112785 * t2867 + 2.0 / 9.0 * t1901 * t112790 * t2877 + t112795 + 2.0 / 9.0 * t1901 * t15191 * t24909 + t1901 * t15191 * t24913 / 9.0 + t112803 + 2.0 / 27.0 * t1901 * t98809 * t15395 - 2.0 / 9.0 * t1901 * t15229 * t112807;
    (t112811,)
}
