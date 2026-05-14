//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 995/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk995<F: Float>(t10491: F, t1255: F, t18969: F, t61128: F, t18962: F, t10915: F, t13296: F, t13301: F, t13309: F, t13320: F, t13346: F, t13352: F, t15567: F, t18961: F, t18968: F, t2917: F, t3691: F, t3700: F, t44663: F, t44666: F, t44669: F, t44672: F, t44683: F, t44709: F, t44712: F, t44716: F, t61123: F, t824: F) -> (F, F) {
    let t72443 = t10491 * t1255;
    let t72910 = t61128 * t18969 / 9.0;
    let t72912 = 2.0 / 27.0 * t61128 * t18962;
    let t72943 = 4.0 / 9.0 * t61123 * t18961 * t13320 + t72910 - t72912 + t44663 / 27.0 - t44666 / 18.0 - t44669 / 36.0 - t44672 / 27.0 + t44683 / 18.0 - t15567 * t18968 * t13346 / 2.0 + 2.0 / 3.0 * t15567 * t18961 * t13352 + t44709 / 9.0 - t44712 / 12.0 - t44716 + t15567 * t2917 * t824 * t3700 / 3.0 + t15567 * t18968 * t13296 / 6.0 - 2.0 / 3.0 * t61123 * t18968 * t13301 - 2.0 / 9.0 * t15567 * t10915 * t824 * t3691 - t15567 * t18961 * t13309 / 9.0;
    (t72443, t72943)
}
