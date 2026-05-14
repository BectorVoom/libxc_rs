//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1151/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1151<F: Float>(t28932: F, t7699: F, t27856: F, t27895: F, t15573: F, t2173: F, t28996: F, t100619: F, t100622: F, t101057: F, t19674: F, t2175: F, t27808: F, t27964: F, t3489: F, t7703: F, t8034: F, t8042: F, t95524: F, t96391: F) -> (F,) {
    let t101393 = t28932 * t7699;
    let t101395 = t27895 * t27856;
    let t101402 = t2173 * t15573 * t28996;
    let t101406 = -0.24872916666666666666e-2 * t100619 - 0.33163888888888888888e-2 * t100622 - 0.13901041666666666667e-2 * t7703 * t101057 - 0.55652820312500000001e-3 * t95524 * t27808 + 0.18534722222222222222e-2 * t19674 * t3489 * t2175 - 0.23168402777777777778e-3 * t101393 + 0.6183646701388888889e-4 * t101395 - 0.37069444444444444445e-2 * t27964 * t8042 - 0.49469173611111111112e-3 * t96391 * t8034 - 0.46336805555555555557e-3 * t101402 - 0.37069444444444444445e-2 * t27964 * t8034;
    (t101406,)
}
