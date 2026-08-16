//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1115/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1115(t10007: f64, t1880: f64, t7394: f64, t9438: f64, t1944: f64, t3240: f64, t3248: f64, t1949: f64, t731: f64, t9625: f64, t21455: f64, t739: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t29078 = t7394 * t9438 * t10007 * t1880;
    let t29160 = 0.19938401337405766662e-2_f64 * t1944 * t3240;
    let t29162 = 0.19938401337405766662e-2_f64 * t1944 * t3248;
    let t29184 = 0.17090058289204942853e-2_f64 * t1949 * t3248;
    let t29186 = 0.17090058289204942853e-2_f64 * t731 * t9625;
    let t29190 = t739 * t21455;
    (t29078, t29160, t29162, t29184, t29186, t29190)
}
