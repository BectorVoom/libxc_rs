//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 896/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk896(t30882: f64, t1998: f64, t3697: f64, t1997: f64, t3243: f64, t390: f64, t7796: f64, t7799: f64, t3036: f64, t3213: f64, t1035: f64, t1039: f64, t7613: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t30883 = 0.10940814253092610657e-1_f64 * t30882;
    let t30886 = t1998 * t3697;
    let t30887 = 0.42874018118069736972e-3_f64 * t30886;
    let t30889 = t3243 * t1997 * t390;
    let t30890 = 0.12862205435420921092e-2_f64 * t30889;
    let t30893 = t7799 * t7796;
    let t30904 = t3036 * t1997 * t3213;
    let t30905 = 0.25724410870841842183e-2_f64 * t30904;
    let t30907 = t1035 * t7613 * t1039;
    (t30883, t30887, t30890, t30893, t30905, t30907)
}
