//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 896/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk896<F: Float>(t30882: F, t1998: F, t3697: F, t1997: F, t3243: F, t390: F, t7796: F, t7799: F, t3036: F, t3213: F, t1035: F, t1039: F, t7613: F) -> (F, F, F, F, F, F) {
    let t30883 = F::new(0.10940814253092610657e-1) * t30882;
    let t30886 = t1998 * t3697;
    let t30887 = F::new(0.42874018118069736972e-3) * t30886;
    let t30889 = t3243 * t1997 * t390;
    let t30890 = F::new(0.12862205435420921092e-2) * t30889;
    let t30893 = t7799 * t7796;
    let t30904 = t3036 * t1997 * t3213;
    let t30905 = F::new(0.25724410870841842183e-2) * t30904;
    let t30907 = t1035 * t7613 * t1039;
    (t30883, t30887, t30890, t30893, t30905, t30907)
}
