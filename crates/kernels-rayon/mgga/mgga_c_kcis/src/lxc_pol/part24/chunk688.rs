//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 688/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk688(t393: f64, t8060: f64, t1820: f64, t7740: f64, t2189: f64, t5036: f64) -> (f64, f64, f64, f64) {
    let t8061 = t8060 * t393;
    let t8062 = t7740 * t1820;
    let t8063 = t5036 * t2189;
    let t8064 = t2189 * t1820;
    (t8061, t8062, t8063, t8064)
}
