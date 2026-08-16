//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 502/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk502<F: Float>(t2846: F, t452: F, t2094: F, t2096: F, t2098: F, t2100: F, t2733: F, t2736: F, t2803: F, t2807: F, t447: F, t1748: F, t2116: F, t2121: F, t2124: F, t2783: F, t2796: F, t2813: F, t2817: F, t2826: F, t2829: F, t2832: F, t2835: F, t2838: F, t455: F, t463: F) -> (F, F, F, F, F) {
    let t2847 = t2846 * t452;
    let t2854 = t2094 - F::cast_from(1.4770435158815312_f64) * t2803 + t2096 + F::cast_from(1.4770435158815312_f64) * t2807 + t2098 - F::cast_from(0.2946275542389858_f64) * t2733 + t2100 + F::cast_from(0.2946275542389858_f64) * t2736;
    let t2855 = t447 * t2854;
    let t2856 = t2855 * t452;
    let t2859 = -t2796 * t1748 / F::cast_from(6.0_f64) + t463 * t2783 / F::cast_from(6.0_f64) - t2813 * t1748 / F::cast_from(6.0_f64) - F::cast_from(0.10237773105191754_f64) * t2736 + F::cast_from(0.14975624337724558_f64) * t2817 + F::cast_from(0.10237773105191754_f64) * t2733 + t2826 * t455 / F::cast_from(6.0_f64) + t2829 * t455 / F::cast_from(6.0_f64) + t2832 * t2116 / F::cast_from(12.0_f64) - t2835 * t455 / F::cast_from(6.0_f64) - t2838 * t455 / F::cast_from(6.0_f64) - t2847 * t455 / F::cast_from(6.0_f64) - t2856 * t455 / F::cast_from(6.0_f64) - t2121 - t2124;
    (t2847, t2854, t2855, t2856, t2859)
}
