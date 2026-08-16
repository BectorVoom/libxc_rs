//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 693/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk693(t12868: f64, t597: f64, t1645: f64, t3137: f64, t2859: f64, t3085: f64, t8124: f64, t1445: f64, t4527: f64, t12806: f64, t1562: f64, t1531: f64, t2876: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12870 = 0.11502877786176224903e2_f64 * t597 * t12868;
    let t12871 = t1645 * t3137;
    let t12873 = 0.10725146985555128001e1_f64 * t2859 * t12871;
    let t12874 = t8124 * t3085;
    let t12875 = t1445 * t12874;
    let t12877 = 0.27606906686822939767e2_f64 * t4527 * t12875;
    let t12878 = t1445 * t12806;
    let t12880 = 0.62115540045351614476e2_f64 * t1562 * t12878;
    let t12881 = t2876 * t1531;
    (t12870, t12871, t12873, t12874, t12875, t12877, t12878, t12880, t12881)
}
