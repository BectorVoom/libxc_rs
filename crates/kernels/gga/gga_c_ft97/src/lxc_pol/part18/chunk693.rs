//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 693/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk693<F: Float>(t11064: F, t11863: F, t3214: F, t379: F, t8557: F, t3271: F, t11826: F, t11829: F, t11833: F, t11839: F, t11843: F, t11846: F, t11849: F, t11851: F, t11856: F, t11860: F, t1901: F, t3281: F, t446: F, t8393: F, t8409: F) -> (F, F, F) {
    let t11864 = t11863 * t11064;
    let t11867 = t3214 * t379;
    let t11868 = t8557 * t11867;
    let t11871 = t3271 * t379;
    let t11872 = t8557 * t11871;
    let t11876 = -t11826 - 2.0 / 9.0 * t1901 * t11829 - 2.0 / 9.0 * t1901 * t11833 - 2.0 / 27.0 * t8393 - 2.0 / 3.0 * t446 * t11839 - 2.0 / 9.0 * t3281 * t11843 - 4.0 / 27.0 * t11846 + t11849 - 2.0 / 9.0 * t446 * t11851 - 4.0 / 9.0 * t1901 * t11856 - 2.0 / 9.0 * t1901 * t11860 - 4.0 / 9.0 * t1901 * t11864 - 2.0 / 9.0 * t1901 * t11868 - 2.0 / 9.0 * t1901 * t11872 - 2.0 / 9.0 * t8409;
    (t11867, t11871, t11876)
}
