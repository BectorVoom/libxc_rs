//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1267/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1267<F: Float>(t3262: F, t3465: F, t43757: F, t11189: F, t3275: F, t43721: F, t3472: F, t43729: F, t11336: F, t37327: F, t42868: F, t1146: F, t2881: F, t2995: F, t3570: F, t3781: F, t44882: F, t44885: F, t44888: F, t44893: F, t44897: F, t44899: F, t44902: F, t44904: F, t44907: F, t44909: F, t9832: F) -> (F, F, F, F, F) {
    let t44912 = F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t3262 * t3465 * t43757;
    let t44915 = F::cast_from(45.0_f64) / F::cast_from(64.0_f64) * t3275 * t11189 * t43721;
    let t44918 = F::cast_from(15.0_f64) / F::cast_from(16.0_f64) * t3262 * t3472 * t43729;
    let t44921 = F::cast_from(15.0_f64) / F::cast_from(8.0_f64) * t37327 * t11336 * t42868;
    let t44922 = t1146 * t9832 + F::cast_from(2.0_f64) * t2881 * t3781 + t2995 * t3570 - t44882 - t44885 - t44888 - t44893 - t44897 - t44899 + t44902 + t44904 + t44907 + t44909 - t44912 + t44915 - t44918 + t44921;
    (t44912, t44915, t44918, t44921, t44922)
}
