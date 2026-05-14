//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 837/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk837<F: Float>(t1383: F, t992: F, t153: F, t156: F, t168: F, t242: F, t245: F, t5580: F, t5585: F, t5592: F, t7976: F, t7981: F, t8038: F, t8042: F, t8047: F, t8050: F, t8051: F, t8053: F, t8057: F) -> (F,) {
    let t8058 = t992 * t1383;
    let t8061 = -0.11938374665504764976e-1 * t168 * t245 * t7976 + 0.13287210228946179141e1 * t7981 + 0.42708890021612718669e0 * t153 * t156 * t8038 - 0.16752564107100880375e0 * t8042 - 0.56945186695483624892e0 * t5580 - t8047 + t8050 + 0.16752564107100880375e0 * t8051 - 0.83762820535504401876e-1 * t8053 * t242 - t8057 - 0.83762820535504401876e-1 * t8058 + 0.26574420457892358282e1 * t5585 + t5592;
    (t8061,)
}
