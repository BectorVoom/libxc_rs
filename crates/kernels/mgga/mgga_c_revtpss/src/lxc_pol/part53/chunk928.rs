//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 928/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk928<F: Float>(t27883: F, t786: F, t7286: F, t1903: F, t7274: F, t7296: F, t25902: F, t25905: F, t25914: F, t25919: F, t25921: F, t25941: F, t25948: F, t25951: F, t27885: F, t27889: F, t27891: F, t27896: F, t7295: F, t7921: F) -> (F, F) {
    let t27899 = t786 * t27883;
    let t27900 = t27899 * t7286;
    let t27902 = t7274 * t1903;
    let t27903 = t7296 * t27902;
    let t27907 = F::cast_from(0.12851425765524037203e-1_f64) * t25902 - F::cast_from(0.72280234901709995518e-2_f64) * t25905 - F::cast_from(0.54878743191129263322e-2_f64) * t25914 - t25919 - F::cast_from(0.12851425765524037203e-1_f64) * t27885 + F::cast_from(0.72280234901709995518e-2_f64) * t27889 - F::cast_from(0.12851425765524037203e-1_f64) * t27891 + F::cast_from(0.8673628188205199462e0_f64) * t25921 * t7921 + F::cast_from(0.8673628188205199462e0_f64) * t7295 * t27896 + F::cast_from(0.72280234901709995518e-2_f64) * t27900 + F::cast_from(0.8673628188205199462e0_f64) * t7295 * t27903 - t25941 + t25948 - F::cast_from(0.12851425765524037203e-1_f64) * t25951;
    (t27903, t27907)
}
