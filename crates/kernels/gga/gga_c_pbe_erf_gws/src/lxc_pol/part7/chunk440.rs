//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 440/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk440<F: Float>(t1671: F, t1786: F, t1877: F, t1929: F, t1365: F, t153: F, t274: F, t542: F, t745: F, t1452: F, t156: F, t1596: F, t1598: F, t1601: F, t1602: F, t1605: F, t1608: F, t1611: F, t1613: F, t168: F, t242: F, t245: F) -> (F, F) {
    let t1931 = t1671 + t1786 + t1877 + t1929;
    let t1937 = 0.13287210228946179141e1 * t153 * t1365 * t274;
    let t1939 = t153 * t542 * t745;
    let t1944 = -t1596 + 0.16752564107100880375e0 * t1598 + t1601 - 0.83762820535504401876e-1 * t1602 * t242 - 0.16752564107100880375e0 * t1605 - t1608 - t1611 + 0.39794582218349216586e-1 * t1613 - 0.11938374665504764976e-1 * t168 * t245 * t1931 + t1937 - 0.11389037339096724978e1 * t1939 + 0.42708890021612718669e0 * t153 * t156 * t1452;
    (t1931, t1944)
}
