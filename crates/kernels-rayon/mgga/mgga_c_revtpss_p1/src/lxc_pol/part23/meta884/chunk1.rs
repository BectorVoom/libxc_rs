//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2797/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2797(t1882: f64, t5710: f64, t2782: f64, t4086: f64, t543: f64, t74973: f64, t1398: f64, t6888: f64, t786: f64, t4104: f64, t23037: f64, t10022: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t75198 = t5710 * t1882;
    let t75205 = t2782 * t4086 * t74973 * t543;
    let t75215 = t2782 * t4086 * t6888 * t1398 * t543;
    let t75219 = t2782 * t4086 * t75198 * t543;
    let t75251 = t786 * t4086 * t6888;
    let t75252 = t75251 * t4104;
    let t75267 = t23037 * t1398;
    let t75269 = t2782 * t10022 * t75267;
    (t75205, t75215, t75219, t75251, t75252, t75269)
}
