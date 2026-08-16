//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1215/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1215(t3216: f64, t6818: f64, t11094: f64, t1958: f64, t13487: f64, t1877: f64, t1915: f64, t193: f64, t202: f64, t23285: f64, t23290: f64, t23295: f64, t2379: f64, t2522: f64, t2553: f64, t2745: f64, t2749: f64, t4314: f64, t6666: f64, t6670: f64, t776: f64, t868: f64, t870: f64) -> (f64, f64, f64) {
    let t23738 = t6818 * t3216;
    let t23742 = t1958 * t11094;
    let t23772 = t193 * t202 * t23285 * t870 - 6.0_f64 * t13487 * t2522 * t6670 - 2.0_f64 * t1877 * t23290 * t868 + 2.0_f64 * t1877 * t23295 * t2749 - t1877 * t2745 * t6670 + 6.0_f64 * t1915 * t2379 * t4314 + 3.0_f64 * t1915 * t2522 * t2553 + 6.0_f64 * t2522 * t6666 * t776;
    (t23738, t23742, t23772)
}
