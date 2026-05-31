//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1042/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1042<F: Float>(t2889: F, t535: F, t513: F, t1134: F, t1536: F, t1543: F, t1549: F, t1556: F, t2829: F, t2838: F, t2869: F, t2876: F, t3661: F, t3688: F, t3733: F, t510: F, t7602: F, t7817: F, t9485: F, t9490: F, t9598: F, t9604: F, t9715: F, t9718: F, t9737: F, t9739: F, t9742: F, t9747: F, t9750: F, t9757: F) -> (F, F, F) {
    let t9761 = t535 * t2889;
    let t9762 = t9761 * t513;
    let t9764 = -F::cast_from(88.0_f64) / F::cast_from(27.0_f64) * t2829 * t9485 + F::cast_from(400.0_f64) / F::cast_from(27.0_f64) * t3661 * t9490 + F::cast_from(400.0_f64) / F::cast_from(27.0_f64) * t3733 * t9490 + F::cast_from(64.0_f64) / F::cast_from(27.0_f64) * t3688 * t9715 + F::cast_from(32.0_f64) / F::cast_from(9.0_f64) * t2838 * t9718 - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t7602 * t1556 + F::cast_from(252.0_f64) * t9737 * t9739 + F::cast_from(12.0_f64) * t9742 * t9739 + F::cast_from(400.0_f64) / F::cast_from(9.0_f64) * t9598 * t9604 + F::cast_from(2.0_f64) * t9747 * t1536 - F::cast_from(24.0_f64) * t510 * t9750 * t2876 + F::cast_from(120.0_f64) * t7817 * t1543 * t2869 + F::cast_from(252.0_f64) * t1134 * t9757 * t2876 + t9762 * t1549;
    (t9761, t9762, t9764)
}
