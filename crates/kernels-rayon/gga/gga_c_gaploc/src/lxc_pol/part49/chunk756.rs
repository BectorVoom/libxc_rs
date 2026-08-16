//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 756/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk756(t12865: f64, t597: f64, t12766: f64, t1445: f64, t1645: f64, t3137: f64, t2859: f64, t3085: f64, t8124: f64, t4527: f64, t12806: f64, t1562: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12866 = t597 * t12865;
    let t12868 = t1445 * t12766;
    let t12870 = 0.11502877786176224903e2_f64 * t597 * t12868;
    let t12871 = t1645 * t3137;
    let t12873 = 0.10725146985555128001e1_f64 * t2859 * t12871;
    let t12874 = t8124 * t3085;
    let t12875 = t1445 * t12874;
    let t12877 = 0.27606906686822939767e2_f64 * t4527 * t12875;
    let t12878 = t1445 * t12806;
    let t12880 = 0.62115540045351614476e2_f64 * t1562 * t12878;
    (t12866, t12868, t12870, t12871, t12873, t12874, t12875, t12877, t12878, t12880)
}
