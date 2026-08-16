//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 2027/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2027<F: Float>(t91064: F, t91076: F, t91078: F, t91081: F, t3787: F, t7918: F, t1336: F, t1814: F, t24116: F, t24121: F, t3793: F, t5230: F, t5287: F, t7211: F, t81187: F, t81189: F, t81197: F, t81216: F, t81218: F, t81230: F, t91048: F, t91052: F, t91074: F, t91091: F) -> F {
    let t93792 = F::cast_from(0.15352717957250113407e0_f64) * t91064;
    let t93794 = F::cast_from(0.76763589786250567036e-1_f64) * t91076;
    let t93795 = F::cast_from(0.52089578783527170489e-1_f64) * t91078;
    let t93796 = F::cast_from(0.3289868133696452873e-1_f64) * t91081;
    let t93798 = t3787 * t7918;
    let t93809 = -F::cast_from(0.51175726524167044691e0_f64) * t81187 + F::cast_from(0.15352717957250113407e0_f64) * t81189 + F::cast_from(0.6579736267392905746e-1_f64) * t81197 + F::cast_from(0.19739208802178717238e0_f64) * t91048 - F::cast_from(0.39478417604357434476e0_f64) * t91052 + F::cast_from(0.16449340668482264365e-1_f64) * t81216 + F::cast_from(0.76763589786250567036e-1_f64) * t81218 + t93792 + F::cast_from(0.3289868133696452873e-1_f64) * t91074 + t93794 - t93795 + t93796 - F::cast_from(0.3289868133696452873e-1_f64) * t81230 + F::cast_from(2.0_f64) * t1336 * t93798 * t3793 + F::cast_from(0.16449340668482264365e-1_f64) * t91091 + t1814 * t24121 + F::cast_from(2.0_f64) * t5230 * t7211 - F::cast_from(2.0_f64) * t1336 * t24116 * t5287;
    t93809
}
