//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 520/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk520<F: Float>(t274: F, t4731: F, t1684: F, t45: F, t1692: F, t3005: F, t1226: F, t2919: F, t2968: F, t3013: F, t3020: F, t4612: F, t4615: F, t4618: F, t4623: F, t4658: F, t4660: F, t4701: F, t4703: F, t4706: F, t4709: F, t4712: F, t4716: F) -> (F, F, F, F, F) {
    let t4732 = t4731 * t274;
    let t4735 = t45 * t1684;
    let t4740 = t3005 * t1692;
    let t4741 = t4740 * t1226;
    let t4758 = -F::cast_from(0.1294625e1_f64) * t4658 + F::cast_from(0.258925e1_f64) * t4660 + t3013 + F::cast_from(0.10064166666666666667e0_f64) * t2919 + F::cast_from(0.10064166666666666667e0_f64) * t4612 - F::cast_from(0.20128333333333333333e0_f64) * t4615 + F::cast_from(0.60385e0_f64) * t4618 - F::cast_from(0.60385e0_f64) * t4623 + F::cast_from(0.82524375e-1_f64) * t4701 + F::cast_from(0.16504875e0_f64) * t4703 + t3020 + F::cast_from(0.5519e-1_f64) * t2968 + F::cast_from(0.5519e-1_f64) * t4706 - F::cast_from(0.27595e-1_f64) * t4709 + F::cast_from(0.16557e0_f64) * t4712 - F::cast_from(0.16557e0_f64) * t4716;
    (t4732, t4735, t4740, t4741, t4758)
}
