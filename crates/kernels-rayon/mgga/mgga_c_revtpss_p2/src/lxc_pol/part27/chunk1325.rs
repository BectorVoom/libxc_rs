//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1325/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1325(t26983: f64, t7658: f64, t3153: f64, t97010: f64, t12627: f64, t7635: f64, t1203: f64, t12600: f64, t12689: f64, t1294: f64, t13184: f64, t2142: f64, t225: f64, t26941: f64, t26958: f64, t26963: f64, t26969: f64, t26976: f64, t26994: f64, t26999: f64, t27020: f64, t27025: f64, t27028: f64, t29194: f64, t29200: f64, t3551: f64, t3568: f64, t3585: f64, t3738: f64, t3791: f64, t460: f64, t494: f64, t5465: f64, t5480: f64, t7627: f64, t7632: f64, t7636: f64, t7637: f64, t7638: f64, t7643: f64, t7652: f64, t7662: f64, t97299: f64) -> f64 {
    let t97453 = t26983 * t7658;
    let t97458 = t97010 * t3153;
    let t97475 = t12627 * t7635;
    let t97480 = -0.19756347548806534796e1_f64 * t26999 * t3585 - 0.19756347548806534796e1_f64 * t27020 * t3791 + 0.65854491829355115987e0_f64 * t460 * t97299 * t225 * t494 - 0.15612530738769359031e2_f64 * t7636 * t26969 * t7638 * t3738 - 0.26020884564615598386e1_f64 * t7636 * t7637 * t7627 * t3551 + 0.10408353825846239354e2_f64 * t26994 * t7637 * t27028 * t1203 - 0.8673628188205199462e0_f64 * t7636 * t7637 * t2142 * t12689 - 0.26020884564615598386e1_f64 * t97453 * t7662 - 0.26020884564615598386e1_f64 * t27025 * t26963 - 0.26020884564615598386e1_f64 * t29194 * t97458 * t5465 + 0.13010442282307799193e1_f64 * t29200 * t97458 * t5480 - 0.39512695097613069591e1_f64 * t7632 * t13184 - 0.52041769129231196772e1_f64 * t7643 * t7652 * t26958 * t1294 - 0.39512695097613069591e1_f64 * t26976 * t12600 - 0.52041769129231196772e1_f64 * t27025 * t26941 - 0.15612530738769359031e2_f64 * t97475 * t7637 * t7638 * t3568;
    t97480
}
