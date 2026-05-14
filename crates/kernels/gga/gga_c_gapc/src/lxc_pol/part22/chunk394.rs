//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 394/1209 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk394<F: Float>(t213: F, t218: F, t215: F, t690: F, t211: F, t414: F, t88: F, t220: F, t694: F, t43: F, t231: F, t4: F, t1220: F, t283: F, t482: F, t132: F, t762: F, t737: F, zeta_threshold: F) -> (F, F, F, F, F) {
    let t214 = t213 <= zeta_threshold;
    let t219 = t218 <= zeta_threshold;
    let t2013 = t215 * t215;
    let t2014 = 1.0 / t2013;
    let t2015 = t690 * t690;
    let t2018 = t211 * t414;
    let t2020 = -2.0 * t88 + 2.0 * t2018;
    let t2024 = piecewise3(t214, 0.0, 4.0 / 9.0 * t2014 * t2015 + 4.0 / 3.0 * t215 * t2020);
    let t2025 = t220 * t220;
    let t2026 = 1.0 / t2025;
    let t2027 = t694 * t694;
    let t2030 = -t2020;
    let t2034 = piecewise3(t219, 0.0, 4.0 / 9.0 * t2026 * t2027 + 4.0 / 3.0 * t220 * t2030);
    let t2036 = (t2024 + t2034) * t43;
    let t2040 = t231 * t4;
    let t2042 = 0.10843580882781524214e-1 * t2040 * t1220;
    let t2043 = t482 * t283;
    let t2046 = t132 * t762;
    let t2053 = t88 * t737;
    (t2036, t2042, t2043, t2046, t2053)
}
