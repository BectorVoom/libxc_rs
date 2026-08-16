//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 442/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk442(t2244: f64, t2980: f64, t2979: f64, t337: f64, t39: f64, t1887: f64, t60: f64, t976: f64, t984: f64, t343: f64, t883: f64, t607: f64) -> (f64, f64, f64, f64, f64) {
    let t2981 = t2980 * t2244;
    let t2982 = t2979 * t2981;
    let t2985 = t39 * t337;
    let t2986 = t2985 * t1887;
    let t2987 = t60 * t976;
    let t2988 = t2987 * t984;
    let t2989 = t343 * t883;
    let t2990 = t2989 * t607;
    (t2982, t2986, t2987, t2988, t2990)
}
