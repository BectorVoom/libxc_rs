//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1407/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1407(t23734: f64, t3216: f64, t11094: f64, t6818: f64, t1958: f64, t43637: f64, t10121: f64, t10140: f64, t13487: f64, t1877: f64, t1914: f64, t1915: f64, t193: f64, t202: f64, t23286: f64, t23290: f64, t23295: f64, t2379: f64, t2522: f64, t2553: f64, t2745: f64, t2749: f64, t4314: f64, t46240: f64, t46252: f64, t46298: f64, t46320: f64, t46362: f64, t6666: f64, t6670: f64, t776: f64, t81525: f64, t81539: f64, t82307: f64, t82312: f64, t868: f64, t870: f64, t9458: f64, t9516: f64, t9616: f64) -> (f64, f64, f64, f64) {
    let t83468 = t23734 * t3216;
    let t83472 = t6818 * t11094;
    let t83479 = t1958 * t43637;
    let t83543 = -18.0_f64 * t4314 * t6670 * t46298 + 18.0_f64 * t2522 * t23295 * t46320 - 3.0_f64 * t1877 * t23290 * t2745 - 18.0_f64 * t2522 * t23290 * t13487 + 18.0_f64 * t4314 * t1915 * t9616 + 9.0_f64 * t2522 * t6666 * t2553 - 9.0_f64 * t2522 * t6670 * t46252 - 9.0_f64 * t2522 * t6670 * t46240 + 6.0_f64 * t1877 * t81539 * t2749 + t193 * t202 * t82307 * t870 + 9.0_f64 * t2522 * t23286 * t776 - 6.0_f64 * t1877 * t82312 * t10140 + 6.0_f64 * t1877 * t23295 * t46362 - 3.0_f64 * t1877 * t81525 * t868 + 6.0_f64 * t193 * t9458 * t1914 * t870 + 18.0_f64 * t4314 * t6666 * t2379 - t1877 * t6670 * t10121 + 3.0_f64 * t2522 * t1915 * t9516;
    (t83468, t83472, t83479, t83543)
}
