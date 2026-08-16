//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 690/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk690(t10961: f64, t1852: f64, t1820: f64, t979: f64, t3219: f64, t8466: f64, t1851: f64, t971: f64, t1853: f64, t1904: f64, t2983: f64, t7793: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10962 = t1852 * t10961;
    let t10964 = t979 * t1820;
    let t10965 = t1852 * t10964;
    let t10967 = t8466 * t3219;
    let t10969 = t971 * t1851;
    let t10970 = t10969 * t1853;
    let t10974 = t2983 * t1904;
    let t10975 = t7793 * t10974;
    (t10962, t10965, t10967, t10970, t10974, t10975)
}
