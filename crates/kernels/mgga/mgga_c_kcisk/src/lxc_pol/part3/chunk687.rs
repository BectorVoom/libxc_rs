//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 687/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk687<F: Float>(t1640: F, t4741: F, t4745: F, t4740: F, t583: F, t573: F, t10560: F, t4744: F, t10570: F, t10572: F, t10574: F, t10576: F, t10587: F, t10595: F, t10607: F, t10610: F, t10613: F, t10615: F, t10617: F, t10619: F, t10623: F, t10626: F) -> (F, F, F) {
    let t10710 = t1640 * t4741;
    let t10712 = F::cast_from(0.48245472966453314466e2_f64) * t10710 * t4745;
    let t10714 = F::new(1.0) / t4740 / t583;
    let t10715 = t573 * t10714;
    let t10716 = t10560 * t4744;
    let t10718 = F::cast_from(0.96490945932906628932e2_f64) * t10715 * t10716;
    let t10733 = -F::cast_from(0.32862666666666666666e0_f64) * t10607 + F::cast_from(0.16431333333333333333e0_f64) * t10610 - F::cast_from(0.49293999999999999999e0_f64) * t10613 - F::cast_from(0.27385555555555555556e0_f64) * t10615 + F::cast_from(0.16431333333333333333e0_f64) * t10617 + F::cast_from(0.5477111111111111111e-1_f64) * t10619 - F::cast_from(0.36514074074074074075e-1_f64) * t10623 - F::cast_from(0.82156666666666666667e-1_f64) * t10626 - F::cast_from(0.59793333333333333333e0_f64) * t10587 + F::new(0.17938e1) * t10595 - F::cast_from(0.39862222222222222223e0_f64) * t10570 + F::cast_from(0.19931111111111111111e0_f64) * t10572 - F::cast_from(0.59793333333333333333e0_f64) * t10574 + F::cast_from(0.29896666666666666667e0_f64) * t10576;
    (t10712, t10718, t10733)
}
