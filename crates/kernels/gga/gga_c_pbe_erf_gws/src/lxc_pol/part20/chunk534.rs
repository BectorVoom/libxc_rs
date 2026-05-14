//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 534/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk534<F: Float>(t153: F, t156: F, t1596: F, t1598: F, t1601: F, t1605: F, t1608: F, t1611: F, t1613: F, t168: F, t1937: F, t1939: F, t242: F, t245: F, t2520: F, t2523: F, t2526: F, t2531: F, t2831: F, t2837: F, t2848: F) -> (F,) {
    let t2852 = -t1596 + 0.83762820535504401876e-1 * t1598 + t1601 + 0.83762820535504401876e-1 * t2520 - 0.83762820535504401876e-1 * t2523 * t242 - 0.83762820535504401876e-1 * t2526 - 0.83762820535504401876e-1 * t1605 - t1608 - t1611 + 0.19897291109174608293e-1 * t1613 + 0.19897291109174608293e-1 * t2531 - 0.11938374665504764976e-1 * t168 * t245 * t2831 + t1937 - 0.56945186695483624892e0 * t1939 - 0.56945186695483624892e0 * t2837 + 0.42708890021612718669e0 * t153 * t156 * t2848;
    (t2852,)
}
