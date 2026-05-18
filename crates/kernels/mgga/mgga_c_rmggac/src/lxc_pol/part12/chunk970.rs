//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 970/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk970<F: Float>(t2604: F, t8997: F, t2024: F, t5249: F, t1356: F, t27044: F, t27120: F, t29892: F, t35777: F, t35782: F, t35787: F, t40554: F, t40556: F, t40559: F, t40561: F, t40563: F, t40565: F, t40567: F, t40568: F, t40573: F, t40575: F, t739: F, t7703: F, t884: F) -> (F, F) {
    let t40578 = t2604 * t8997;
    let t40579 = F::new(0.79828278012425390426e-1) * t40578;
    let t40589 = t2024 * t5249;
    let t40592 = -t35777 - t35782 + t35787 + F::new(0.53205749866622299248e-5) * t40554 - F::new(0.12769379967989351819e-4) * t40556 - t40559 + t40561 - t40563 - t40565 + t40567 + F::new(0.1064114997332445985e-4) * t40568 + F::new(0.1064114997332445985e-4) * t40573 + F::new(0.11974241701863808564e0) * t884 * t40575 - t40579 + F::new(0.23948483403727617128e0) * t739 * t2024 * t29892 - F::new(0.23948483403727617128e0) * t884 * t2024 * t27044 - F::new(0.23948483403727617128e0) * t1356 * t7703 * t27120 + F::new(0.39914139006212695214e-1) * t1356 * t40589;
    (t40589, t40592)
}
