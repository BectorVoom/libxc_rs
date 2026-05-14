//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 763/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk763<F: Float>(t1426: F, t27836: F, t7063: F, t7286: F, t72: F, t7929: F, t686: F, t7284: F, t7289: F, t1444: F, t7296: F, t7910: F, t786: F, t1903: F, t7274: F, t25902: F, t25905: F, t25914: F, t25919: F, t25921: F, t25941: F, t25948: F, t25951: F, t7295: F, t7921: F) -> (F, F, F, F) {
    let t27883 = t27836 * t1426;
    let t27884 = t7063 * t27883;
    let t27885 = t27884 * t7286;
    let t27887 = t7929 * t72;
    let t27888 = t27887 * t686;
    let t27889 = t7284 * t27888;
    let t27891 = t7289 * t27888;
    let t27896 = t7296 * t7910 * t1444;
    let t27899 = t786 * t27883;
    let t27900 = t27899 * t7286;
    let t27902 = t7274 * t1903;
    let t27903 = t7296 * t27902;
    let t27907 = 0.12851425765524037203e-1 * t25902 - 0.72280234901709995518e-2 * t25905 - 0.54878743191129263322e-2 * t25914 - t25919 - 0.12851425765524037203e-1 * t27885 + 0.72280234901709995518e-2 * t27889 - 0.12851425765524037203e-1 * t27891 + 0.8673628188205199462e0 * t25921 * t7921 + 0.8673628188205199462e0 * t7295 * t27896 + 0.72280234901709995518e-2 * t27900 + 0.8673628188205199462e0 * t7295 * t27903 - t25941 + t25948 - 0.12851425765524037203e-1 * t25951;
    (t27888, t27896, t27903, t27907)
}
