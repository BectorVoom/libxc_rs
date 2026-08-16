//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1266/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1266(t92876: f64, t92932: f64, t93149: f64, t93201: f64, t93250: f64, t93299: f64, t93345: f64, t93393: f64, t892: f64, t11064: f64, t7086: f64, t1940: f64, t1963: f64, t1964: f64, t2403: f64, t25198: f64, t25208: f64, t25215: f64, t25436: f64, t25440: f64, t25446: f64, t25449: f64, t25452: f64, t30: f64, t4541: f64, t7010: f64, t7087: f64, t7091: f64, t92795: f64, t92799: f64, t92806: f64, t92810: f64, t92814: f64, t92819: f64, t92822: f64, t9344: f64) -> (f64, f64, f64, f64) {
    let t93396 = t92876 + t92932 + t93149 + t93201 + t93250 + t93299 + t93345 + t93393;
    let t93397 = t93396 * t892;
    let t93404 = t7086 * t11064;
    let t93408 = 9.0_f64 / 2.0_f64 * t2403 * t1963 * t92795 + 9.0_f64 / 2.0_f64 * t2403 * t1963 * t92799 + 9.0_f64 / 2.0_f64 * t2403 * t25436 * t7010 + 9.0_f64 * t4541 * t1963 * t92806 - t1940 * t7091 * t92810 / 2.0_f64 + 3.0_f64 / 2.0_f64 * t2403 * t1963 * t92814 - 9.0_f64 * t92819 * t25208 + 3.0_f64 * t92822 * t1964 + 9.0_f64 * t4541 * t7087 * t25198 - 3.0_f64 / 2.0_f64 * t1940 * t25440 * t25452 + 9.0_f64 / 2.0_f64 * t2403 * t7087 * t25215 + t1940 * t1963 * t9344 / 2.0_f64 + t1940 * t93397 * t30 / 2.0_f64 - 3.0_f64 * t1940 * t25440 * t25449 + 3.0_f64 * t1940 * t93404 * t25446;
    (t93396, t93397, t93404, t93408)
}
