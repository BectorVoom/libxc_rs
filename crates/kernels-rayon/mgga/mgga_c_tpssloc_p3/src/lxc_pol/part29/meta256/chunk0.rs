//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1192/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1192(t1351: f64, t562: f64, t550: f64, t6976: f64, t1992: f64, t1372: f64, t1998: f64, t214: f64, t1985: f64, t1338: f64, t2006: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t6977 = t562 * t1351;
    let t6978 = t6977 * t550;
    let t6979 = t6976 * t6978;
    let t6980 = t1992 * t6979;
    let t6982 = t1998 * t1372;
    let t6983 = t214 * t6982;
    let t6984 = t1985 * t6983;
    let t6987 = t1338 * t2006;
    (t6978, t6979, t6980, t6982, t6983, t6984, t6987)
}
