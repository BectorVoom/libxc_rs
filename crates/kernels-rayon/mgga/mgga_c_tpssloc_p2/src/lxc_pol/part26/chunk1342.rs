//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1342/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1342(t5: f64, t85479: f64, t85504: f64, t85532: f64, t85569: f64, t112: f64, t2319: f64, t7263: f64, t11968: f64, t12492: f64, t12504: f64, t1266: f64, t2114: f64, t2165: f64, t2167: f64, t2314: f64, t2320: f64, t2323: f64, t24543: f64, t24545: f64, t24932: f64, t3652: f64, t3929: f64, t510: f64, t7264: f64, t7266: f64, t7271: f64, t7408: f64, t7412: f64, t81419: f64, t81422: f64, t81426: f64, t81430: f64, t81432: f64, t81434: f64, t81458: f64, t9348: f64, t9351: f64) -> (f64, f64, f64) {
    let t7 = piecewise3(0.0_f64 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0_f64;
    let t85572 = piecewise3(t8, 0.0_f64, t85479 + t85504 + t85532 + t85569);
    let t85573 = t85572 * t112;
    let t85577 = t7263 * t2319;
    let t85585 = -t81419 - 6.0_f64 * t7266 * t12504 - 6.0_f64 * t9348 * t7271 - 12.0_f64 * t2314 * t24545 - 12.0_f64 * t24932 * t2323 + t81422 - 6.0_f64 * t2320 * t7408 + t81426 - t81430 - t81432 - t81434 - t81458 - 3.0_f64 * t7264 * t3652 - t2114 * t11968 - t85573 * t510 - 3.0_f64 * t24543 * t1266 - 6.0_f64 * t85577 * t510 - 6.0_f64 * t9351 * t2165 + 3.0_f64 * t7412 * t3929 + t2167 * t12492;
    (t85573, t85577, t85585)
}
