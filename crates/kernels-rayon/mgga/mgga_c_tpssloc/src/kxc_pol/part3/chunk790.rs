//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 790/1255 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk790(t381: f64, t4552: f64, t1049: f64, t1603: f64, t1604: f64, t225: f64, t1625: f64, t990: f64, t4343: f64, t977: f64, t2979: f64, t4338: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4553 = t4552 * t381;
    let t4555 = t1603 * t1049;
    let t4557 = t1604 * t225;
    let t4559 = t990 * t1625;
    let t4562 = t977 * t4343;
    let t4565 = t2979 * t4338;
    (t4553, t4555, t4557, t4559, t4562, t4565)
}
