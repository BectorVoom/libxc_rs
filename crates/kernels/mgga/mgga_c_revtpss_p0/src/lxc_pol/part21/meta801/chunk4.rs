//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2911/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2911<F: Float>(t2967: F, t4644: F, t11449: F, t1614: F, t15373: F, t945: F, t11409: F, t1621: F, t2968: F, t11445: F, t11453: F, t11456: F, t11466: F, t11467: F, t11513: F, t11517: F, t11525: F, t15104: F, t15235: F, t15339: F, t15350: F, t15400: F, t15406: F, t1622: F, t1634: F, t2963: F, t2970: F, t2971: F, t2982: F, t41794: F, t4647: F, t4708: F, t953: F, t955: F) -> F {
    let t52820 = t4644 * t2967;
    let t52825 = t1614 * t11449;
    let t52830 = t15373 * t945;
    let t52837 = t11409 * t1621;
    let t52840 = t2968 * t1621;
    let t52856 = F::cast_from(0.17544670867903938621e1_f64) * t11456 * t4708 + F::cast_from(0.96491876992155210402e2_f64) * t52820 * t2971 + F::new(1.0) * t4647 * t11445 + F::cast_from(0.2069040516770936012e4_f64) * t52825 * t11453 + F::new(1.0) * t41794 * t1622 + F::new(3.0) * t52830 * t955 + F::new(3.0) * t15400 * t2963 + F::cast_from(0.17544670867903938621e1_f64) * t2982 * t15235 - F::cast_from(0.57895126195293126243e3_f64) * t52837 * t11517 + F::new(18.0) * t52840 * t11513 + F::cast_from(0.96491876992155210402e2_f64) * t2968 * t15339 * t2970 * t953 + F::cast_from(0.51947577317044391277e2_f64) * t15350 * t11525 - F::cast_from(0.14035736694323150897e2_f64) * t11466 * t1634 * t11467 - F::new(6.0) * t15104 * t11513 + F::cast_from(0.96491876992155210402e2_f64) * t15406 * t11517;
    t52856
}
