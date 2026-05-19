//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 781/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk781<F: Float>(t5520: F, t5522: F, t5525: F, t5539: F, t665: F, t5519: F, t210: F, t5512: F, t1873: F, t667: F, t1867: F, t672: F) -> (F, F, F, F, F, F, F) {
    let t5540 = -t5520 + F::new(4.0) / F::new(3.0) * t5522 - t5525 + t5539;
    let t5541 = t665 * t5540;
    let t5543 = F::cast_from(0.93932222222222222223e0_f64) * t5519;
    let t5547 = F::new(1.0)/pow_3_2::<F>(t210);
    let t5548 = t5547 * t5512;
    let t5550 = t1873 * t667;
    let t5551 = t5550 * t1867;
    let t5553 = t672 * t5540;
    (t5540, t5541, t5543, t5547, t5548, t5551, t5553)
}
