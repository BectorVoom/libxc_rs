//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 689/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk689(t2253: f64, t2920: f64, t2941: f64, t3312: f64, t3682: f64, t4026: f64, t4399: f64, t1853: f64, t979: f64, t8418: f64, t3255: f64, t492: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10925 = t2253 * t2920;
    let t10927 = t2253 * t2941;
    let t10947 = 2.0_f64 * t3312;
    let t10948 = 2.0_f64 * t3682;
    let t10949 = 2.0_f64 * t4026;
    let t10950 = 2.0_f64 * t4399;
    let t10951 = t979 * t1853;
    let t10952 = t8418 * t10951;
    let t10961 = t3255 * t492;
    (t10925, t10927, t10947, t10948, t10949, t10950, t10952, t10961)
}
