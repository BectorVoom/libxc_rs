//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 907/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk907(t2374: f64, t55: f64, t8655: f64, t8656: f64, t2310: f64, t661: f64, t662: f64, t2339: f64, t2309: f64, t2333: f64, t663: f64, t2349: f64, t671: f64, t8630: f64, t8631: f64, t8634: f64, t8637: f64, t8640: f64, t8646: f64, t8649: f64, t8653: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8659 = 1.0_f64 / t2374 / t55;
    let t8660 = t8655 * t8656 * t8659;
    let t8663 = t2310 * t661;
    let t8664 = t8663 * t662;
    let t8666 = 6.0_f64 * t2339 * t8664;
    let t8669 = 6.0_f64 * t2309 * t663 * t2333;
    let t8670 = t8630 - 0.32530742648344572643e-1_f64 * t2349 * t8631 - 0.21687161765563048428e-1_f64 * t2349 * t8634 + 0.16265371324172286321e-1_f64 * t2349 * t8637 + 0.48159446095139119799e0_f64 * t2349 * t8640 + t8646 - t8649 - t8653 - 0.1025389702100779493e4_f64 * t671 * t8660 + t8666 - t8669;
    (t8659, t8660, t8663, t8666, t8669, t8670)
}
