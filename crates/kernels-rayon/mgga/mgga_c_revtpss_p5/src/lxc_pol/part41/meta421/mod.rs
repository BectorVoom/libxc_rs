//! MGGA_C_REVTPSS lxc pol kernel — _part41_v4rho3tau_4 meta421 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1477;
use chunk1::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1478;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_meta421(t31555: f64, t508: f64, t569: f64, t1911: f64, t8362: f64, t1843: f64, t1312: f64, t18245: f64, t2179: f64, t2181: f64, t29508: f64, t30138: f64, t30143: f64, t31518: f64, t31533: f64, t4248: f64, t651: f64, t7732: f64, t7889: f64, t8353: f64, t8363: f64, t8367: f64, t8369: f64, t3: f64, t2178: f64, t5883: f64, t1518: f64, t31370: f64, t5920: f64, t8295: f64, t117: f64, t1916: f64, t1918: f64, t2187: f64, t2189: f64, t572: f64, t573: f64, t6941: f64, t6945: f64, t6948: f64, t8377: f64, t8383: f64, t8386: f64, param_d: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t31556, t31567, t31570, t31579, t31582) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1477(t31555, t508, t569, t1911, t8362, t1843, t1312, t18245, t2179, t2181, t29508, t30138, t30143, t31518, t31533, t4248, t651, t7732, t7889, t8353, t8363, t8367, t8369);
        let (t31583, t31593, t31607, t31610, t31613, t31616, t31619) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1478(t3, t31582, t2178, t5883, t1518, t31370, t5920, t8295, t117, t31555, t1916, t1918, t2187, t2189, t572, t573, t6941, t6945, t6948, t8377, t8383, t8386, param_d);
    (t31556, t31567, t31570, t31579, t31582, t31583, t31593, t31607, t31610, t31613, t31616, t31619)
}
