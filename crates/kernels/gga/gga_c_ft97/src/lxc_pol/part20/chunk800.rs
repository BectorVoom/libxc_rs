//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 800/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk800<F: Float>(t1501: F, t2739: F, t840: F, t871: F, t2749: F, t6365: F, t6386: F, t824: F, t1901: F, t24870: F, t24875: F, t24879: F, t24882: F, t24884: F, t24887: F, t24891: F, t24895: F, t24900: F, t24903: F, t24905: F, t24910: F, t24914: F, t24918: F, t446: F) -> (F, F, F, F, F, F) {
    let t24921 = t1501 * t2739;
    let t24923 = t840 * t871 * t24921;
    let t24927 = t840 * t2749 * t6365;
    let t24930 = t6386 * t824;
    let t24932 = t840 * t871 * t24930;
    let t24935 = 2.0 / 27.0 * t1901 * t24870 - 2.0 / 9.0 * t1901 * t24875 - 4.0 / 9.0 * t1901 * t24879 - 2.0 / 9.0 * t24882 - 4.0 / 9.0 * t24884 + 2.0 / 9.0 * t1901 * t24887 + 2.0 / 9.0 * t1901 * t24891 - 2.0 / 9.0 * t1901 * t24895 - 4.0 / 3.0 * t1901 * t24900 - 2.0 / 27.0 * t24903 + 2.0 / 9.0 * t1901 * t24905 + 2.0 / 9.0 * t1901 * t24910 + t1901 * t24914 / 9.0 + 4.0 / 3.0 * t446 * t24918 + t446 * t24923 / 3.0 + 2.0 / 3.0 * t446 * t24927 + 2.0 / 3.0 * t446 * t24932;
    (t24921, t24923, t24927, t24930, t24932, t24935)
}
