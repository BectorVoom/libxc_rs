//! MGGA_C_REVTPSS lxc pol kernel — _part41_v4rho3tau_4 meta421 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1477;
use chunk1::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1478;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_meta421<F: Float>(t31555: F, t508: F, t569: F, t1911: F, t8362: F, t1843: F, t1312: F, t18245: F, t2179: F, t2181: F, t29508: F, t30138: F, t30143: F, t31518: F, t31533: F, t4248: F, t651: F, t7732: F, t7889: F, t8353: F, t8363: F, t8367: F, t8369: F, t3: F, t2178: F, t5883: F, t1518: F, t31370: F, t5920: F, t8295: F, t117: F, t1916: F, t1918: F, t2187: F, t2189: F, t572: F, t573: F, t6941: F, t6945: F, t6948: F, t8377: F, t8383: F, t8386: F, param_d: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t31556, t31567, t31570, t31579, t31582) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1477::<F>(t31555, t508, t569, t1911, t8362, t1843, t1312, t18245, t2179, t2181, t29508, t30138, t30143, t31518, t31533, t4248, t651, t7732, t7889, t8353, t8363, t8367, t8369);
        let (t31583, t31593, t31607, t31610, t31613, t31616, t31619) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1478::<F>(t3, t31582, t2178, t5883, t1518, t31370, t5920, t8295, t117, t31555, t1916, t1918, t2187, t2189, t572, t573, t6941, t6945, t6948, t8377, t8383, t8386, param_d);
    (t31556, t31567, t31570, t31579, t31582, t31583, t31593, t31607, t31610, t31613, t31616, t31619)
}
