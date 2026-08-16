//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 602/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk602(t10556: f64, t544: f64, t2392: f64, t2482: f64, t2890: f64, t9267: f64, t2299: f64, t2875: f64, t1424: f64, t4130: f64, t986: f64, t9272: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10557 = t544 * t10556;
    let t10559 = 0.42900587942220512003e1_f64 * t10557 * t2392;
    let t10597 = t2890 * t2482;
    let t10598 = t9267 * t10597;
    let t10599 = 0.9585731488480187419e0_f64 * t10598;
    let t10600 = t2299 * t2875;
    let t10601 = t544 * t10600;
    let t10603 = 0.39722766613167140743e-1_f64 * t10601 * t1424;
    let t10608 = t4130 * t986;
    let t10609 = t10608 * t2482;
    let t10610 = t9272 * t10609;
    (t10557, t10559, t10599, t10600, t10603, t10608, t10610)
}
