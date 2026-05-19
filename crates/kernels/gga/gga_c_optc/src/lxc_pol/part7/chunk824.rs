//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 824/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk824<F: Float>(t2488: F, t2513: F, t2521: F, t2525: F, t2531: F, t2534: F, t2538: F, t7675: F, t7678: F, t7684: F, t7691: F, t7753: F, t7754: F, t7759: F, t7760: F, t7794: F, t7799: F, t7802: F, t7805: F, t7810: F, t7813: F, t7814: F, t7817: F, t7820: F, t7825: F, t7828: F, t810: F, t819: F, t829: F, t838: F) -> F {
    let t7831 = -t7675 + t7678 + t7684 - t7691 + F::cast_from(0.1025389702100779493e4_f64) * t7753 * t7754 - F::cast_from(0.19298809906722418785e3_f64) * t7759 * t7760 + F::new(1.0) * t810 * t7794 + F::cast_from(0.20691336878655965246e4_f64) * t7799 * t7802 + F::cast_from(0.17544670192365612213e1_f64) * t7805 * t838 + F::cast_from(0.17544670192365612213e1_f64) * t2525 * t2534 + F::cast_from(0.51947267698127589899e2_f64) * t7810 * t2538 - F::cast_from(0.1038945353962551798e3_f64) * t7813 * t7814 + F::cast_from(0.58482233974552040708e0_f64) * t829 * t7817 + F::new(3.0) * t7820 * t819 + F::new(3.0) * t2488 * t2513 + F::cast_from(0.96494049533612093922e2_f64) * t7825 * t2521 - F::cast_from(0.35089340384731224426e1_f64) * t7828 * t2531;
    t7831
}
