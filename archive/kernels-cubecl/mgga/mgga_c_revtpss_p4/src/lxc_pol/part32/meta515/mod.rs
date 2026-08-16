//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta515 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1817;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta515<F: Float>(t30555: F, t30625: F, t3: F, t2055: F, t5883: F, t1518: F, t28986: F, t5920: F, t7553: F, t117: F, t30570: F, t1916: F, t1918: F, t2113: F, t2115: F, t572: F, t573: F, t6941: F, t6945: F, t6948: F, t8118: F, t8124: F, t8127: F, param_d: F) -> (F, F, F, F, F, F, F, F) {
        let (t30626, t30627, t30637, t30651, t30654, t30657, t30660, t30663) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1817::<F>(t30555, t30625, t3, t2055, t5883, t1518, t28986, t5920, t7553, t117, t30570, t1916, t1918, t2113, t2115, t572, t573, t6941, t6945, t6948, t8118, t8124, t8127, param_d);
    (t30626, t30627, t30637, t30651, t30654, t30657, t30660, t30663)
}
