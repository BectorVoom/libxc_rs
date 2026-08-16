//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1241/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1241(t32983: f64, t3005: f64, t7344: f64, t7810: f64, t32435: f64, t5241: f64, t5640: f64, t590: f64, t10981: f64, t5771: f64, t1445: f64, t24908: f64, t813: f64, t935: f64) -> (f64, f64, f64, f64, f64) {
    let t32984 = 0.38342925953920749676e0_f64 * t32983;
    let t32986 = t7810 * t3005 * t7344;
    let t32987 = 0.19171462976960374838e0_f64 * t32986;
    let t32991 = 0.30674340763136599742e1_f64 * t5640 * t5241 * t32435 * t590;
    let t32997 = 0.14300195980740170668e1_f64 * t5771 * t10981;
    let t33001 = 0.46011511144704899612e1_f64 * t813 * t1445 * t24908 * t935;
    (t32984, t32987, t32991, t32997, t33001)
}
