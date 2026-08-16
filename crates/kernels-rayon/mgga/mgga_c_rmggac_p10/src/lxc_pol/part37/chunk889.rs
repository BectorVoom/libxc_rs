//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 889/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk889(t14125: f64, t14131: f64, t8431: f64, t739: f64, t74292: f64, t7577: f64, t1326: f64, t15144: f64, t321: f64, t68729: f64, t333: f64, t70585: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t75946 = t14131 * t14125 * t8431;
    let t75951 = 0.5987120850931904282e-1_f64 * t739 * t7577 * t74292;
    let t75953 = t1326 * t15144 * t321;
    let t75954 = t68729 * t75953;
    let t75956 = t15144 * t333;
    let t75957 = t1326 * t75956;
    let t75958 = t70585 * t75957;
    (t75946, t75951, t75953, t75954, t75956, t75957, t75958)
}
