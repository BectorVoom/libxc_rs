//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 624/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk624<F: Float>(t776: F, t2028: F, t791: F, t1992: F, t794: F, t772: F, t41: F, t4794: F, t1758: F, t1995: F, t4973: F, t4977: F, t525: F, t642: F, t773: F) -> (F, F, F, F, F, F, F, F) {
    let t777 = t776 < -F::new(0.66725e-1);
    let t5437 = t2028 * t2028;
    let t5438 = t791 * t791;
    let t5439 = F::new(1.0) / t5438;
    let t5440 = t5437 * t5439;
    let t5444 = F::new(1.0) / t1992 / t794;
    let t5445 = t772 * t5444;
    let t5449 = t4794 * t41;
    let t5463 = piecewise3::<f64>(t777, F::new(0.0), F::new(10.0) / F::new(9.0) * t525 * t5449 * t642 - F::new(20.0) / F::new(27.0) * t525 * t1995 * t1758 + F::new(40.0) / F::new(81.0) * t525 * t773 * t4973 - F::new(10.0) / F::new(27.0) * t525 * t773 * t4977);
    (t5437, t5438, t5439, t5440, t5444, t5445, t5449, t5463)
}
