//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1202/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1202<F: Float>(t2409: F, t36089: F, t3959: F, t14001: F, t15331: F, t1178: F, t12169: F, t371: F, t3983: F, t13953: F, t15345: F, t35654: F, t3909: F, t3955: F, t13796: F, t13859: F, t3896: F, t875: F) -> (F, F, F, F, F, F, F) {
    let t57694 = t3959 * t2409 * t36089;
    let t57696 = t14001 * t15331;
    let t57700 = t3983 * t371 * t1178 * t12169;
    let t57702 = t13953 * t15345;
    let t57705 = t3959 * t2409 * t35654;
    let t57707 = t3955 * t3909;
    let t57711 = t13859 * t13796 * t3896 * t875;
    (t57694, t57696, t57700, t57702, t57705, t57707, t57711)
}
