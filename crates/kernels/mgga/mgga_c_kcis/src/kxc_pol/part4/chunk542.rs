//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 542/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk542<F: Float>(t169: F, t180: F, t2791: F, t980: F, t442: F, t911: F, t916: F, t1296: F, t2635: F, t234: F, t441: F, t233: F, t1295: F, t915: F, sigma0: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F) {
    let t170 = t169 <= zeta_threshold;
    let t2792 = t180 * t2791;
    let t2793 = F::cast_from(1.0_f64) / t980;
    let t2794 = sigma0 * t2793;
    let t2795 = t2794 * t442;
    let t2796 = t2795 / F::cast_from(8.0_f64);
    let t2797 = t911 * t916;
    let t2798 = t2797 / F::cast_from(8.0_f64);
    let t2799 = t911 * t1296;
    let t2800 = t2799 / F::cast_from(8.0_f64);
    let t2801 = piecewise3::<F>(t170, F::cast_from(0.0_f64), t2635);
    let t2802 = t234 * t2801;
    let t2803 = t2802 * t441;
    let t2804 = t233 * t2803;
    let t2805 = t2804 / F::cast_from(16.0_f64);
    let t2806 = t915 * t1295;
    (t2792, t2794, t2796, t2798, t2800, t2802, t2805, t2806)
}
