//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 735/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk735<F: Float>(t1268: F, t2923: F, t4973: F, t1091: F, t5468: F, t10864: F, t5457: F, t4969: F, t21181: F, t231: F, t2918: F, t21204: F, t4342: F, t14431: F, t14445: F, t18823: F, t18825: F, t18874: F, t21871: F, t2265: F, t631: F) -> (F, F, F, F, F, F, F, F) {
    let t21875 = t2923 * t4973 * t1268;
    let t21877 = t1091 * t5468;
    let t21878 = t2923 * t21877;
    let t21881 = t10864 * t1091 * t5457;
    let t21885 = t2923 * t4969 * t1268;
    let t21893 = t231 * t2918 * t21181;
    let t21895 = t4342 * t21204;
    let t21897 = t2265 * t21871 / 6.0 - t2265 * t21875 - t2265 * t21878 + 3.0 * t2265 * t21881 + 2.0 * t2265 * t21885 + 5.0 / 9.0 * t14431 - t18823 + 3.0 * t18825 + 5.0 / 3.0 * t14445 - t18874 / 9.0 + t631 * t21893 - t2265 * t21895;
    (t21875, t21877, t21878, t21881, t21885, t21893, t21895, t21897)
}
