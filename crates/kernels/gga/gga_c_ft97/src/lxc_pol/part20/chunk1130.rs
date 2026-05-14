//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1130/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1130<F: Float>(t1403: F, t2399: F, t6843: F, t27957: F, t681: F, t28466: F, t42109: F, t6003: F, t14108: F, t24412: F, t14254: F, t2526: F, t28023: F, t24211: F, t6745: F, t10157: F, t107885: F, t13706: F, t14213: F, t24204: F, t24257: F, t27947: F, t28002: F, t5996: F, t6002: F, t6749: F, t6840: F, t96863: F) -> (F, F, F, F) {
    let t109787 = t1403 * t2399 * t6843;
    let t109793 = 2.0 / 9.0 * t1403 * t681 * t27957;
    let t109798 = 2.0 / 9.0 * t1403 * t681 * t28466;
    let t109799 = t42109 * t6003;
    let t109803 = t24412 * t14108;
    let t109805 = t24412 * t14254;
    let t109807 = t28023 * t2526;
    let t109809 = t6745 * t24211;
    let t109811 = t6002 * t10157 * t6003 * t14213 - t96863 * t6749 / 18.0 - t24204 * t28002 / 9.0 + 2.0 / 27.0 * t109787 + t24257 * t6840 / 6.0 + t109793 + t5996 * t27947 / 3.0 + t109798 + 4.0 / 27.0 * t107885 * t109799 * t13706 + 8.0 * t109803 + 4.0 * t109805 - 2.0 * t109807 + 2.0 / 27.0 * t109809;
    (t109803, t109805, t109807, t109811)
}
