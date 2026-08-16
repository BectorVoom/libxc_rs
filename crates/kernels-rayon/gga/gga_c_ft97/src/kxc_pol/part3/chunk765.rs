//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 765/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk765(t15885: f64, t370: f64, t27: f64, t89: f64, t375: f64, t4496: f64, t4437: f64, t15746: f64, t1866: f64, t3281: f64, t1882: f64, t4423: f64) -> (f64, f64, f64, f64, f64) {
    let t15886 = t370 * t15885;
    let t15888 = t89 * t27 * t15886;
    let t15891 = t89 * t375 * t4496;
    let t15894 = t89 * t375 * t4437;
    let t15896 = t1866 * t15746;
    let t15897 = t3281 * t15896;
    let t15899 = t1882 * t4423;
    (t15888, t15891, t15894, t15897, t15899)
}
