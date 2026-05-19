//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1247/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1247<F: Float>(t3843: F, t940: F, t942: F, t1: F, t23951: F, t24521: F, t24566: F, t24575: F, t24615: F, t24620: F, t25059: F, t25095: F, t25562: F, t25730: F, t25740: F, t25742: F, t25751: F, t25753: F, t25769: F, t2644: F, t2648: F, t2758: F, t297: F, t313: F, t894: F, t914: F, t930: F, t935: F, t953: F) -> F {
    let t25772 = t940 * t3843 * t942;
    let t25774 = F::cast_from(0.59710464543246456043e-1_f64) * t25730 + F::cast_from(0.30050434779516693818e0_f64) * t930 * t914 * t24620 + F::cast_from(0.28977204965962526182e-1_f64) * t930 * t914 * t24615 + F::cast_from(0.30228422675018518373e0_f64) * t953 * t25095 + F::cast_from(0.20606012420240018619e0_f64) * t25740 + F::cast_from(0.75587607063262836759e5_f64) * t25742 * t25562 * t935 * t2644 - F::cast_from(0.30228422675018518374e-1_f64) * t953 * t25059 - F::cast_from(0.61944912485988186948e2_f64) * t25751 - F::cast_from(0.67174272611152263053e-2_f64) * t25753 - F::cast_from(0.40304563566691357832e-1_f64) * t953 * t894 * t2648 * t23951 - F::cast_from(0.27821325036192187983e8_f64) * t24566 * t313 * t24575 * t935 - F::cast_from(0.23229342182245570105e2_f64) * t2758 * t313 * t24521 * t1 * t297 + F::cast_from(0.10324152080998031158e2_f64) * t25769 + F::cast_from(0.69310201356862480534e1_f64) * t25772;
    t25774
}
