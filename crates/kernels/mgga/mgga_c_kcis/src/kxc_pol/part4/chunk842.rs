//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 842/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk842<F: Float>(t2374: F, t55: F, t8655: F, t8656: F, t2310: F, t661: F, t662: F, t2339: F, t2309: F, t2333: F, t663: F, t2349: F, t671: F, t8630: F, t8631: F, t8634: F, t8637: F, t8640: F, t8646: F, t8649: F, t8653: F) -> (F, F, F, F, F, F) {
    let t8659 = 1.0 / t2374 / t55;
    let t8660 = t8655 * t8656 * t8659;
    let t8663 = t2310 * t661;
    let t8664 = t8663 * t662;
    let t8666 = 6.0 * t2339 * t8664;
    let t8669 = 6.0 * t2309 * t663 * t2333;
    let t8670 = t8630 - 0.32530742648344572643e-1 * t2349 * t8631 - 0.21687161765563048428e-1 * t2349 * t8634 + 0.16265371324172286321e-1 * t2349 * t8637 + 0.48159446095139119799e0 * t2349 * t8640 + t8646 - t8649 - t8653 - 0.1025389702100779493e4 * t671 * t8660 + t8666 - t8669;
    (t8659, t8660, t8663, t8666, t8669, t8670)
}
