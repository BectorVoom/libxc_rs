//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 1011/1059 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk1011<F: Float>(t225: F, t33948: F, t114592: F, t114606: F, t121296: F, t121299: F, t121302: F, t121305: F, t121308: F, t121311: F, t121314: F, t121318: F, t121326: F, t13042: F, t2054: F, t866: F, t8741: F, t92847: F, t92939: F) -> F {
    let t123443 = t33948 * t225;
    let t123452 = -F::cast_from(0.3289868133696452873e-1_f64) * t114592 + F::cast_from(0.76763589786250567037e-1_f64) * t121296 + F::cast_from(0.6579736267392905746e-1_f64) * t121299 - F::cast_from(0.3289868133696452873e-1_f64) * t121302 + F::cast_from(0.16449340668482264365e-1_f64) * t121305 - F::cast_from(0.3289868133696452873e-1_f64) * t121308 - F::cast_from(0.6579736267392905746e-1_f64) * t121311 - F::cast_from(0.6579736267392905746e-1_f64) * t121314 - F::cast_from(0.3289868133696452873e-1_f64) * t121318 - t123443 * t866 - F::cast_from(2.0_f64) * t92847 * t2054 - F::cast_from(0.13159472534785811492e0_f64) * t121326 - F::cast_from(0.15352717957250113407e0_f64) * t114606 - t13042 * t8741 - F::cast_from(2.0_f64) * t92939 * t2054;
    t123452
}
