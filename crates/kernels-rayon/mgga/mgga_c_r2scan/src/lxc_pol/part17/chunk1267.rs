//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1267/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1267(t3262: f64, t3465: f64, t43757: f64, t11189: f64, t3275: f64, t43721: f64, t3472: f64, t43729: f64, t11336: f64, t37327: f64, t42868: f64, t1146: f64, t2881: f64, t2995: f64, t3570: f64, t3781: f64, t44882: f64, t44885: f64, t44888: f64, t44893: f64, t44897: f64, t44899: f64, t44902: f64, t44904: f64, t44907: f64, t44909: f64, t9832: f64) -> (f64, f64, f64, f64, f64) {
    let t44912 = 3.0_f64 / 2.0_f64 * t3262 * t3465 * t43757;
    let t44915 = 45.0_f64 / 64.0_f64 * t3275 * t11189 * t43721;
    let t44918 = 15.0_f64 / 16.0_f64 * t3262 * t3472 * t43729;
    let t44921 = 15.0_f64 / 8.0_f64 * t37327 * t11336 * t42868;
    let t44922 = t1146 * t9832 + 2.0_f64 * t2881 * t3781 + t2995 * t3570 - t44882 - t44885 - t44888 - t44893 - t44897 - t44899 + t44902 + t44904 + t44907 + t44909 - t44912 + t44915 - t44918 + t44921;
    (t44912, t44915, t44918, t44921, t44922)
}
