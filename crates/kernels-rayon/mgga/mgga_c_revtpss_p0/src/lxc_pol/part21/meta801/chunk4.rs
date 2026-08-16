//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2911/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2911(t2967: f64, t4644: f64, t11449: f64, t1614: f64, t15373: f64, t945: f64, t11409: f64, t1621: f64, t2968: f64, t11445: f64, t11453: f64, t11456: f64, t11466: f64, t11467: f64, t11513: f64, t11517: f64, t11525: f64, t15104: f64, t15235: f64, t15339: f64, t15350: f64, t15400: f64, t15406: f64, t1622: f64, t1634: f64, t2963: f64, t2970: f64, t2971: f64, t2982: f64, t41794: f64, t4647: f64, t4708: f64, t953: f64, t955: f64) -> f64 {
    let t52820 = t4644 * t2967;
    let t52825 = t1614 * t11449;
    let t52830 = t15373 * t945;
    let t52837 = t11409 * t1621;
    let t52840 = t2968 * t1621;
    let t52856 = 0.17544670867903938621e1_f64 * t11456 * t4708 + 0.96491876992155210402e2_f64 * t52820 * t2971 + 1.0_f64 * t4647 * t11445 + 0.2069040516770936012e4_f64 * t52825 * t11453 + 1.0_f64 * t41794 * t1622 + 3.0_f64 * t52830 * t955 + 3.0_f64 * t15400 * t2963 + 0.17544670867903938621e1_f64 * t2982 * t15235 - 0.57895126195293126243e3_f64 * t52837 * t11517 + 18.0_f64 * t52840 * t11513 + 0.96491876992155210402e2_f64 * t2968 * t15339 * t2970 * t953 + 0.51947577317044391277e2_f64 * t15350 * t11525 - 0.14035736694323150897e2_f64 * t11466 * t1634 * t11467 - 6.0_f64 * t15104 * t11513 + 0.96491876992155210402e2_f64 * t15406 * t11517;
    t52856
}
