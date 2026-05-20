//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta422 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1484;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta422<F: Float>(t3: F, t31700: F, t2198: F, t5883: F, t1518: F, t31505: F, t5920: F, t8342: F, t117: F, t31653: F, t1916: F, t1918: F, t2207: F, t2209: F, t572: F, t573: F, t6941: F, t6945: F, t6948: F, t8421: F, t8427: F, t8430: F, param_d: F) -> (F, F, F, F, F, F, F) {
        let (t31701, t31711, t31725, t31728, t31731, t31734, t31737) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1484::<F>(t3, t31700, t2198, t5883, t1518, t31505, t5920, t8342, t117, t31653, t1916, t1918, t2207, t2209, t572, t573, t6941, t6945, t6948, t8421, t8427, t8430, param_d);
    (t31701, t31711, t31725, t31728, t31731, t31734, t31737)
}
