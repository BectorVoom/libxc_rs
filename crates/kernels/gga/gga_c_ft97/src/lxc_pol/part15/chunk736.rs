//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 736/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk736<F: Float>(t21870: F, t21897: F, t332: F, t113: F, t10214: F, t4917: F, t10222: F, t2639: F, t4635: F, t231: F, t5299: F, t10207: F, t1526: F, t18959: F, t18977: F, t2320: F, t342: F, t343: F, t3806: F, t5207: F, t5213: F, t5305: F) -> (F, F, F, F, F, F, F, F) {
    let t21898 = t21870 + t21897;
    let t21899 = t21898 * t332;
    let t21900 = t21899 * t113;
    let t21911 = t10214 * t4917;
    let t21918 = t10222 * t4917;
    let t21922 = t2639 * t4635;
    let t21926 = t231 * t5299;
    let t21930 = t5207 + t5305 + t10207 - t18959 / 18.0 - t18977 / 6.0 - t1526 * t3806 * t21911 / 9.0 - t1526 * t2320 * t5213 / 6.0 + t1526 * t2320 * t21918 / 6.0 - t1526 * t2320 * t21922 / 12.0 - t342 * t343 * t21926 / 4.0;
    (t21898, t21899, t21900, t21911, t21918, t21922, t21926, t21930)
}
