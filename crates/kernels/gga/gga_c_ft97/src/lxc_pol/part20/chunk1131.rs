//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1131/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1131<F: Float>(t24223: F, t6745: F, t2469: F, t27889: F, t2526: F, t2568: F, t6940: F, t28163: F, t8392: F, t10052: F, t676: F, t28341: F, t28188: F, t108020: F, t108024: F, t108028: F, t108045: F, t108280: F, t108395: F, t108401: F, t11593: F, t14163: F, t14182: F, t14187: F, t14196: F, t14200: F, t1901: F, t2347: F, t2360: F, t2579: F, t3886: F, t53504: F, t6187: F, t67996: F, t6861: F, t6930: F) -> (F, F, F, F) {
    let t109822 = t6745 * t24223 / 9.0;
    let t109823 = t2469 * t27889;
    let t109827 = t2568 * t6940 * t2526;
    let t109844 = 4.0 / 27.0 * t8392 * t28163;
    let t109848 = t676 * t10052;
    let t109863 = 4.0 / 27.0 * t8392 * t28341;
    let t109875 = 2.0 / 27.0 * t8392 * t28188;
    let t109885 = 8.0 / 3.0 * t1901 * t67996 * t6861 * t2579 + t109844 + 10.0 / 81.0 * t1901 * t53504 * t108395 + 4.0 * t1901 * t109848 * t6930 * t2579 - 2.0 / 9.0 * t1901 * t14196 * t108020 - 4.0 / 9.0 * t1901 * t14200 * t108024 + 2.0 / 27.0 * t1901 * t14200 * t108280 + t109863 - 4.0 / 9.0 * t1901 * t14182 * t6187 * t2360 * t3886 + 4.0 / 27.0 * t1901 * t14187 * t6187 * t2347 * t3886 + t109875 + 4.0 / 9.0 * t1901 * t14163 * t108045 + 8.0 / 9.0 * t11593 * t14196 * t108028 - 8.0 / 27.0 * t11593 * t14200 * t108401;
    (t109822, t109823, t109827, t109885)
}
