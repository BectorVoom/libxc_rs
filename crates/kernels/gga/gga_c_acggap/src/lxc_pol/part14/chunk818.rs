//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 818/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk818<F: Float>(t30248: F, t438: F, t30318: F, t425: F, t431: F, t1973: F, t7780: F, t1985: F, t30179: F, t1998: F, t3697: F, t1997: F, t3243: F, t390: F, t7796: F, t7799: F) -> (F, F, F, F, F, F, F, F) {
    let t30872 = t30248 * t438;
    let t30874 = t30318 * t425;
    let t30878 = t30318 * t431;
    let t30880 = t7780 * t1973;
    let t30882 = t30179 * t1985;
    let t30883 = 0.10940814253092610657e-1 * t30882;
    let t30886 = t1998 * t3697;
    let t30887 = 0.42874018118069736972e-3 * t30886;
    let t30889 = t3243 * t1997 * t390;
    let t30890 = 0.12862205435420921092e-2 * t30889;
    let t30893 = t7799 * t7796;
    (t30872, t30874, t30878, t30880, t30883, t30887, t30890, t30893)
}
