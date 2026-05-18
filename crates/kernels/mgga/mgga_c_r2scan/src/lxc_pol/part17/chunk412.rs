//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 412/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk412<F: Float>(t14: F, t1906: F, t1647: F, t653: F, t621: F, t632: F, t645: F, t190: F, t650: F, t1743: F, t225: F, t664: F) -> (F, F, F, F, F, F) {
    let t1907 = t14 * t1906;
    let t1910 = F::new(0.96491876992155210402e2) * t1907 * t653 * t1647;
    let t1913 = F::new(4.0) * t632 * t645 * t621;
    let t1916 = F::new(6.0) * t650 * t190 * t1647;
    let t1917 = t1743 * t225;
    let t1923 = t664 * t664;
    (t1907, t1910, t1913, t1916, t1917, t1923)
}
