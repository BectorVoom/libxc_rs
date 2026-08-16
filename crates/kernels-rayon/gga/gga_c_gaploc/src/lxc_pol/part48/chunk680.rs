//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 680/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk680(t123: f64, t3431: f64, t883: f64, t969: f64, t825: f64, t2685: f64, t2684: f64, t3247: f64, t900: f64, t10867: f64, t10924: f64, t787: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t13063 = t3431 * t123;
    let t13064 = t13063 * t883;
    let t13065 = t969 * t13064;
    let t13066 = t825 * t13065;
    let t13069 = t2685 * t13064;
    let t13070 = t2684 * t13069;
    let t13072 = t900 * t3247;
    let t13073 = t10867 * t13072;
    let t13077 = t787 * t10924;
    (t13064, t13065, t13066, t13069, t13070, t13072, t13073, t13077)
}
