//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 868/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk868<F: Float>(t1696: F, t40067: F, t538: F, t554: F, t2030: F, t550: F, t12411: F, t1698: F, t1995: F, t2003: F, t2031: F, t2032: F, t23847: F, t399: F, t40055: F, t40059: F, t40068: F, t40069: F, t40078: F, t40081: F, t40084: F, t40087: F, t527: F, t8838: F, t8885: F, t8998: F, t9001: F) -> (F, F, F) {
    let t40090 = t40067 * t1696 * t554 * t538;
    let t40093 = t550 * t2030;
    let t40099 = -0.14498192132169191472e2 * t8838 * t40055 + 0.14498192132169191472e2 * t23847 * t40059 - 0.14498192132169191472e2 * t12411 * t2031 * t399 + 0.14498192132169191472e2 * t9001 * t399 - 0.17516464591774387196e2 * t40068 * t40069 * t2003 - 0.4832730710723063824e1 * t1995 * t8998 * t399 - 0.91821883503738212655e2 * t23847 * t40078 + 0.28996384264338382944e2 * t40081 * t40084 + 0.70065858367097548785e2 * t40087 * t40090 - 0.14498192132169191472e2 * t527 * t40093 * t8885 + 0.17516464591774387197e2 * t2032 * t1698;
    (t40090, t40093, t40099)
}
