//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 667/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk667<F: Float>(t274: F, t4731: F, t1684: F, t45: F, t1692: F, t3005: F, t1226: F, t2919: F, t2968: F, t3013: F, t3020: F, t4612: F, t4615: F, t4618: F, t4623: F, t4658: F, t4660: F, t4701: F, t4703: F, t4706: F, t4709: F, t4712: F, t4716: F) -> (F, F, F, F, F) {
    let t4732 = t4731 * t274;
    let t4735 = t45 * t1684;
    let t4740 = t3005 * t1692;
    let t4741 = t4740 * t1226;
    let t4758 = -F::new(0.1294625e1) * t4658 + F::new(0.258925e1) * t4660 + t3013 + F::new(0.10064166666666666667e0) * t2919 + F::new(0.10064166666666666667e0) * t4612 - F::new(0.20128333333333333333e0) * t4615 + F::new(0.60385e0) * t4618 - F::new(0.60385e0) * t4623 + F::new(0.82524375e-1) * t4701 + F::new(0.16504875e0) * t4703 + t3020 + F::new(0.5519e-1) * t2968 + F::new(0.5519e-1) * t4706 - F::new(0.27595e-1) * t4709 + F::new(0.16557e0) * t4712 - F::new(0.16557e0) * t4716;
    (t4732, t4735, t4740, t4741, t4758)
}
