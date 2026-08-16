//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1867/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1867(t1527: f64, t6662: f64, t2718: f64, t225: f64, t7492: f64, t1484: f64, t857: f64, t865: f64, t23270: f64, t22986: f64, t13065: f64, t13463: f64, t1528: f64, t1912: f64, t23206: f64, t23209: f64, t23231: f64, t23232: f64, t23278: f64, t4268: f64, t4273: f64, t6627: f64, t6632: f64, t6663: f64, t855: f64, t866: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t25183 = t6662 * t1527;
    let t25184 = t2718 * t25183;
    let t25188 = t7492 * t225;
    let t25191 = t857 * t1484;
    let t25192 = t25191 * t865;
    let t25193 = t23270 * t25192;
    let t25194 = t22986 * t25193;
    let t25196 = -t13463 * t1912 - t4268 * t6663 + 0.82246703342411321824e-2_f64 * t23206 + 0.41123351671205660912e-2_f64 * t23209 - t23278 * t1528 + 2.0_f64 * t4268 * t6632 + 2.0_f64 * t6627 * t4273 + 2.0_f64 * t855 * t25184 - t23231 - t13065 * t1912 - t25188 * t866 + 0.38381794893125283518e-1_f64 * t23232 + 0.16449340668482264365e-1_f64 * t25194;
    (t25184, t25188, t25191, t25192, t25193, t25196)
}
