//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 803/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk803<F: Float>(t5737: F, t684: F, t1899: F, t1971: F, t1976: F, t2874: F, t730: F, t5519: F, t5522: F, t5525: F, t5539: F, t228: F) -> (F, F, F, F, F, F, F) {
    let t5738 = t5737 * t684;
    let t5740 = F::new(6.0) * t1899 * t5738;
    let t5742 = t1976 * t1971 * t2874;
    let t5744 = F::new(0.51947577317044391277e2) * t730 * t5742;
    let t5745 = F::new(0.55403703703703703703e-1) * t5519;
    let t5749 = -t5745 + F::new(0.71233333333333333332e-1) * t5522 - F::new(0.53424999999999999999e-1) * t5525 + F::new(0.53425e-1) * t5539;
    let t5751 = F::new(0.621814e-1) * t5749 * t228;
    (t5738, t5740, t5742, t5744, t5745, t5749, t5751)
}
