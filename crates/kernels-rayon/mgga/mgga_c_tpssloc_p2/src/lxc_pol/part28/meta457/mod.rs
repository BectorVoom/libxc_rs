//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta457 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1662;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1663;
use chunk2::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1664;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta457(t25: f64, t265: f64, t394: f64, t24379: f64, t2064: f64, t2250: f64, t24355: f64, t40: f64, t607: f64, t7131: f64, t1081: f64, t1877: f64, t2057: f64, t23781: f64, t23789: f64, t23792: f64, t23796: f64, t23807: f64, t23810: f64, t23813: f64, t24191: f64, t24335: f64, t24339: f64, t24344: f64, t2522: f64, t28: f64, t3231: f64, t4314: f64, t6841: f64, t6848: f64, t7110: f64, t7114: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64, t504: f64, t2071: f64, t52: f64, t7150: f64, t2094: f64, t3701: f64, rho1: f64, t15904: f64, t2075: f64, t2363: f64, t113: f64, t12823: f64, t1983: f64, t2040: f64, t2096: f64, t22574: f64, t22607: f64, t2312: f64, t2314: f64, t2320: f64, t23958: f64, t24008: f64, t24026: f64, t24028: f64, t24167: f64, t24169: f64, t24176: f64, t4034: f64, t510: f64, t574: f64, t650: f64, t652: f64, t6876: f64, t7050: f64, t7057: f64, t7156: f64, t7171: f64, t7218: f64, t7220: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t24380, t24387, t24419) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1662(t25, t265, t394, t24379, t2064, t2250, t24355, t40, t607, t7131, t1081, t1877, t2057, t23781, t23789, t23792, t23796, t23807, t23810, t23813, t24191, t24335, t24339, t24344, t2522, t28, t3231, t4314, t6841, t6848, t7110, t7114, dens_threshold, rho0, zeta_threshold);
        let (t24420, t24428, t24432) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1663(t28, t265, t504, t24379, t2071, t2250, t24419, t52, t607, t7150, t24387, t2094, t3701, dens_threshold, rho1, zeta_threshold);
        let (t24433, t24442, t24446) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1664(t15904, t24432, t2075, t2363, t113, t12823, t1983, t2040, t2096, t22574, t22607, t2312, t2314, t2320, t23958, t24008, t24026, t24028, t24167, t24169, t24176, t24428, t4034, t510, t574, t650, t652, t6876, t7050, t7057, t7156, t7171, t7218, t7220);
    (t24380, t24420, t24428, t24432, t24433, t24442, t24446)
}
