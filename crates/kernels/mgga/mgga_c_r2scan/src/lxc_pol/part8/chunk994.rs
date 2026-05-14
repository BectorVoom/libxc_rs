//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 994/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk994<F: Float>(t1020: F, t2410: F, t2956: F, t839: F, t333: F, t9707: F, t2958: F, t335: F, t337: F, t1022: F, t1024: F, t2960: F, t2962: F, t2964: F, t339: F, t341: F) -> (F, F, F, F) {
    let t9709 = t1020 * t2410;
    let t9711 = t839 * t2956;
    let t9713 = t333 * t9707;
    let t9715 = t839 * t2958;
    let t9721 = t335 * t9707;
    let t9729 = t337 * t9707;
    let t9731 = -0.64e0 * t9707 - 0.17408e1 * t9709 - 0.8704e0 * t9711 - 0.8704e0 * t9713 - 0.9214113627294e1 * t9715 - 0.18428227254588e2 * t1022 * t2410 - 0.9214113627294e1 * t2960 * t839 - 0.4607056813647e1 * t9721 + 0.734774460522e2 * t2962 * t839 + 0.734774460522e2 * t1024 * t2410 + 0.367387230261e2 * t2964 * t839 + 0.122462410087e2 * t9729;
    let t9738 = t339 * t9707;
    let t9746 = t341 * t9707;
    (t9711, t9731, t9738, t9746)
}
