//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 1080/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk1080<F: Float>(t11782: F, t1819: F, t1947: F, t2855: F, t2042: F, t2846: F, t2845: F, t305: F, t2035: F, t11679: F, t451: F, t11102: F, t11733: F, t11767: F, t11773: F, t11776: F, t11778: F, t1748: F, t2032: F, t2053: F, t2104: F, t2111: F, t2114: F, t2783: F, t2838: F, t2847: F, t2856: F, t472: F, t6327: F, t7324: F, t7325: F, t7326: F) -> F {
    let t11783 = t1819 * t11782;
    let t11786 = t2855 * t1947;
    let t11787 = t11786 * t2042;
    let t11789 = t2846 * t1947;
    let t11790 = t11789 * t2042;
    let t11794 = t2845 * t305;
    let t11795 = t2035 * t11794;
    let t11798 = t451 * t11679;
    let t11799 = t11798 * t2042;
    let t11805 = -t2111 * t2783 / F::cast_from(6.0_f64) - t472 * t11733 / F::cast_from(6.0_f64) - t2114 * t11767 / F::cast_from(12.0_f64) + t2104 * t2783 / F::cast_from(6.0_f64) - F::cast_from(0.10237773105191754_f64) * t6327 - t7324 - t7325 + t7326 - F::cast_from(0.14975624337724558_f64) * t11773 - F::cast_from(0.14975624337724558_f64) * t11776 + t11778 / F::cast_from(18.0_f64) + t2856 * t2032 / F::cast_from(6.0_f64) - t11783 * t1748 / F::cast_from(6.0_f64) + t11787 / F::cast_from(6.0_f64) + t11790 / F::cast_from(6.0_f64) + t2847 * t2032 / F::cast_from(6.0_f64) - t11795 * t1748 / F::cast_from(6.0_f64) + t11799 / F::cast_from(6.0_f64) + t2838 * t2032 / F::cast_from(6.0_f64) - t2053 * t11102 / F::cast_from(6.0_f64);
    t11805
}
