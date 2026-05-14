//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1115/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1115<F: Float>(t54463: F, t54491: F, t14954: F, t4414: F, t14981: F, t15004: F, t840: F, t54504: F, t54531: F, t54535: F, t54537: F, t54566: F, t4083: F, t8743: F, t54616: F, t15084: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t55781 = 7.0 / 576.0 * t54463;
    let t55796 = 7.0 / 1152.0 * t54491;
    let t55807 = 7.0 / 72.0 * t4414 * t14954;
    let t55809 = 7.0 / 72.0 * t4414 * t14981;
    let t55831 = 7.0 / 144.0 * t840 * t15004;
    let t55833 = 7.0 / 72.0 * t54504;
    let t55841 = 7.0 / 72.0 * t54531;
    let t55850 = 7.0 / 36.0 * t54535;
    let t55851 = 7.0 / 36.0 * t54537;
    let t55863 = 7.0 / 36.0 * t54566;
    let t55884 = 7.0 / 144.0 * t8743 * t4083;
    let t55889 = 7.0 / 1152.0 * t54616;
    let t55901 = 7.0 / 144.0 * t840 * t15084;
    (t55781, t55796, t55807, t55809, t55831, t55833, t55841, t55850, t55851, t55863, t55884, t55889, t55901)
}
