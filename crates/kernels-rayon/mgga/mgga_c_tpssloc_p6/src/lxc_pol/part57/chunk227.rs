//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 227/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk227(t1339: f64, t240: f64, t1336: f64, t241: f64, t557: f64, t67: f64, t68: f64, t248: f64, t836: f64, t555: f64, t236: f64, t552: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t1340 = t1339 * t240;
    let t1341 = t1336 * t1340;
    let t1342 = t241 * t557;
    let t1343 = t1342 * t67;
    let t1347 = t68 * t557;
    let t1358 = t836 * t557 * t248;
    let t1360 = 7.0_f64 / 4608.0_f64 * t555 * t1358;
    let t1361 = t552 * t236;
    (t1340, t1341, t1342, t1343, t1347, t1358, t1360, t1361)
}
