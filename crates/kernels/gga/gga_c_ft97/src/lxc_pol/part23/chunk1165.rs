//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1165/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1165<F: Float>(t2770: F, t7091: F, t29057: F, t8392: F, t29052: F, t29098: F, t1882: F, t29182: F, t29060: F, t29064: F, t29235: F, t7042: F, t8232: F, t309: F, t43524: F, t29270: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t112790 = t2770 * t7091;
    let t112795 = 4.0 / 9.0 * t8392 * t29057;
    let t112803 = 4.0 / 9.0 * t8392 * t29052;
    let t112821 = 2.0 / 27.0 * t8392 * t29098;
    let t112831 = 2.0 / 9.0 * t1882 * t29182;
    let t112848 = 2.0 / 27.0 * t8392 * t29060;
    let t112853 = 2.0 / 27.0 * t8392 * t29064;
    let t112865 = 2.0 / 9.0 * t1882 * t29235;
    let t112866 = t8232 * t7042;
    let t112888 = t43524 * t309;
    let t112898 = 4.0 / 9.0 * t1882 * t29270;
    (t112790, t112795, t112803, t112821, t112831, t112848, t112853, t112865, t112866, t112888, t112898)
}
