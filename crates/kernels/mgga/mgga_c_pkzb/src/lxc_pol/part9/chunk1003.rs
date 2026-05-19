//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1003/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1003<F: Float>(t8054: F, t8066: F, t871: F, t1201: F, t2295: F, t7930: F, t6090: F, t6093: F, t6180: F, t6183: F, t6211: F, t7947: F, t7950: F, t7955: F, t7959: F, t7961: F, t7967: F) -> (F, F, F, F) {
    let t8067 = t8054 + t8066;
    let t8068 = t8067 * t871;
    let t8071 = t1201 * t2295;
    let t8076 = F::new(0.60385e0) * t7930;
    let t8085 = -t6211 + F::cast_from(0.80513333333333333334e0_f64) * t6090 - F::new(0.301925e0) * t6093 - t8076 + F::new(0.905775e0) * t7947 + F::new(0.27595e0) * t7950 + F::new(0.258925e1) * t7959 + F::new(0.16504875e0) * t7961 - F::new(0.16557e0) * t6180 - F::new(0.16557e0) * t6183 + F::cast_from(0.40256666666666666667e0_f64) * t7955 - F::new(0.258925e1) * t7967;
    (t8067, t8068, t8071, t8085)
}
