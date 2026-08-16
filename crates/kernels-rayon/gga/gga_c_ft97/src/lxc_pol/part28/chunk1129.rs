//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1129/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1129(t139214: f64, t139224: f64, t26909: f64, t32897: f64, t3052: f64, t32898: f64, t32899: f64, t3628: f64, t5889: f64, t2: f64, t34918: f64, t1969: f64, t379: f64, t5899: f64) -> (f64, f64, f64) {
    let t148299 = t32897 * t139224 * t139214 * t26909;
    let t148304 = t5889 * t3628 * t32898 * t32899 * t3052;
    let t148306 = t2 * t34918;
    let t148309 = t5899 * t1969 * t148306 * t379;
    (t148299, t148304, t148309)
}
