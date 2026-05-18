//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 870/1200 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk870<F: Float>(t27883: F, t786: F, t7286: F, t1903: F, t7274: F, t7296: F, t25902: F, t25905: F, t25914: F, t25919: F, t25921: F, t25941: F, t25948: F, t25951: F, t27885: F, t27889: F, t27891: F, t27896: F, t7295: F, t7921: F) -> (F, F) {
    let t27899 = t786 * t27883;
    let t27900 = t27899 * t7286;
    let t27902 = t7274 * t1903;
    let t27903 = t7296 * t27902;
    let t27907 = F::new(0.12851425765524037203e-1) * t25902 - F::new(0.72280234901709995518e-2) * t25905 - F::new(0.54878743191129263322e-2) * t25914 - t25919 - F::new(0.12851425765524037203e-1) * t27885 + F::new(0.72280234901709995518e-2) * t27889 - F::new(0.12851425765524037203e-1) * t27891 + F::new(0.8673628188205199462e0) * t25921 * t7921 + F::new(0.8673628188205199462e0) * t7295 * t27896 + F::new(0.72280234901709995518e-2) * t27900 + F::new(0.8673628188205199462e0) * t7295 * t27903 - t25941 + t25948 - F::new(0.12851425765524037203e-1) * t25951;
    (t27903, t27907)
}
