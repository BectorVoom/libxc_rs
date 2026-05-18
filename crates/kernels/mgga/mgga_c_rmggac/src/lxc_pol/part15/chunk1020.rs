//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 1020/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk1020<F: Float>(t1550: F, t30800: F, t7577: F, t30490: F, t903: F, t35972: F, t45556: F, t739: F, t35584: F, t35587: F, t35591: F, t35593: F, t40121: F, t40124: F, t40126: F, t46034: F, t46038: F, t46040: F, t46043: F, t46045: F, t46071: F, t46322: F, t46352: F, t46372: F, t46411: F, t46449: F, t46478: F, t46518: F, t46562: F, t46601: F, t46633: F, t46666: F, t46701: F, t46734: F, t46763: F, t46786: F, t5928: F, t72: F, t82: F, t8801: F) -> F {
    let t46800 = t1550 * t7577 * t30800;
    let t46803 = t903 * t7577 * t30490;
    let t46806 = t739 * t35972 * t45556;
    let t46808 = -F::new(0.25538759935978703638e-4) * t46034 - F::new(0.25538759935978703638e-4) * t46038 - F::new(0.5987120850931904282e-1) * t46040 + F::new(0.8980681276397856423e-1) * t46043 + F::new(0.2993560425465952141e-1) * t46045 + t72 * t82 * (t46071 + t46322 + t46352 + t46372 + t46411 + t46449 + t46478 + t46518 + t46562 + t46601 + t46633 + t46666 + t46701 + t46734 + t46763 + t46786) + F::new(0.59590439850616975157e-4) * t40121 + t40124 + t40126 - F::new(0.2927036860455597649e0) * t35584 + F::new(0.43905552906833964735e0) * t35587 + F::new(0.14635184302277988245e0) * t35591 + t35593 + F::new(0.79828278012425390428e-1) * t5928 * t8801 - F::new(0.5987120850931904282e-1) * t46800 + F::new(0.8980681276397856423e-1) * t46803 + F::new(0.8980681276397856423e-1) * t46806;
    t46808
}
