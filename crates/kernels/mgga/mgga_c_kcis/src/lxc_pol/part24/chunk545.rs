//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 545/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk545<F: Float>(t1142: F, t5189: F, t2919: F, t3537: F, t4612: F, t4615: F, t4618: F, t4623: F, t1211: F, t1823: F, t1219: F, t1831: F, t2968: F, t3557: F, t3564: F, t4658: F, t4660: F, t4701: F, t4703: F, t4706: F, t4709: F, t4712: F, t4716: F) -> (F, F, F, F, F) {
    let t5190 = t1142 * t5189;
    let t5208 = t3537 + 0.57077777777777777777e-2 * t2919 + 0.57077777777777777777e-2 * t4612 - 0.11415555555555555555e-1 * t4615 + 0.34246666666666666666e-1 * t4618 - 0.34246666666666666666e-1 * t4623;
    let t5211 = t1823 * t1211;
    let t5216 = t1831 * t1219;
    let t5233 = -0.17648625e1 * t4658 + 0.3529725e1 * t4660 + t3557 + 0.17215833333333333333e0 * t2919 + 0.17215833333333333333e0 * t4612 - 0.34431666666666666667e0 * t4615 + 0.103295e1 * t4618 - 0.103295e1 * t4623 + 0.31558125e0 * t4701 + 0.6311625e0 * t4703 + t3564 + 0.69463333333333333333e-1 * t2968 + 0.69463333333333333333e-1 * t4706 - 0.34731666666666666667e-1 * t4709 + 0.20839e0 * t4712 - 0.20839e0 * t4716;
    (t5190, t5208, t5211, t5216, t5233)
}
