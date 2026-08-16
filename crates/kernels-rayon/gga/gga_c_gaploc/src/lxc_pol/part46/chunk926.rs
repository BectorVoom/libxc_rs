//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 926/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk926(t21502: f64, t42944: f64, t1841: f64, t21501: f64, t123: f64, t33137: f64, t2563: f64, t9647: f64, t13182: f64, t32215: f64, t5539: f64, t3487: f64, t7284: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t42945 = t21502 * t42944;
    let t42948 = 0.51270174867614828557e-2_f64 * t1841 * t21501 * t42945;
    let t42949 = t33137 * t123;
    let t42951 = t9647 * t42949 * t2563;
    let t42953 = t1841 * t13182;
    let t42954 = 0.85450291446024714264e-3_f64 * t42953;
    let t42956 = t9647 * t5539 * t32215;
    let t42960 = t9647 * t7284 * t3487 * t2563;
    (t42945, t42948, t42951, t42954, t42956, t42960)
}
