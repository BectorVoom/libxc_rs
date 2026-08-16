//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta734 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2506;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2507;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta734(t14869: f64, t9775: f64, t10899: f64, t136: f64, t216: f64, t14786: f64, t231: f64, t40834: f64, t854: f64, t14833: f64, t236: f64, t2453: f64, t9794: f64, t14724: f64, t10722: f64, t4435: f64, t10716: f64, t14757: f64, t10868: f64, t2482: f64, t814: f64, t10845: f64, t14732: f64, t4423: f64, t853: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t50443, t50446, t50451, t50454, t50457) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2506(t14869, t9775, t10899, t136, t216, t14786, t231, t40834, t854, t14833, t236, t2453, t9794);
        let (t50505, t50524, t50532, t50570, t50582, t50583) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2507(t14724, t9775, t10722, t4435, t10716, t14757, t10868, t2482, t814, t10845, t14732, t4423, t853);
    (t50443, t50446, t50451, t50454, t50457, t50505, t50524, t50532, t50570, t50582, t50583)
}
