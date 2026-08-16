//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 835/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk835(t1841: f64, t35440: f64, t44777: f64, t11657: f64, t2554: f64, t7064: f64, t35385: f64, t883: f64, t2932: f64, t9647: f64, t11680: f64, t40820: f64) -> (f64, f64, f64, f64, f64) {
    let t44780 = 0.10254034973522965711e-1_f64 * t1841 * t35440 * t44777;
    let t44785 = t7064 * t11657 * t2554;
    let t44786 = 0.32043859292259267849e-3_f64 * t44785;
    let t44787 = t883 * t35385;
    let t44789 = t9647 * t2932 * t44787;
    let t44790 = 0.64087718584518535698e-3_f64 * t44789;
    let t44792 = t7064 * t11680 * t40820;
    (t44780, t44786, t44787, t44790, t44792)
}
