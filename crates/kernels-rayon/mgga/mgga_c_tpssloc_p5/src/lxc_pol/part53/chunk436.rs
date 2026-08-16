//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 436/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk436(t2978: f64, t974: f64, t337: f64, t39: f64, t1887: f64, t60: f64, t976: f64, t984: f64, t343: f64, t883: f64, t607: f64, t2822: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t2979 = t974 * t2978;
    let t2985 = t39 * t337;
    let t2986 = t2985 * t1887;
    let t2987 = t60 * t976;
    let t2988 = t2987 * t984;
    let t2989 = t343 * t883;
    let t2990 = t2989 * t607;
    let t3003 = 5.0_f64 / 18.0_f64 * t2822;
    (t2979, t2986, t2987, t2988, t2989, t2990, t3003)
}
