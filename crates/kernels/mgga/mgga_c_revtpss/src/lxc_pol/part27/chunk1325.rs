//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1325/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1325<F: Float>(t26983: F, t7658: F, t3153: F, t97010: F, t12627: F, t7635: F, t1203: F, t12600: F, t12689: F, t1294: F, t13184: F, t2142: F, t225: F, t26941: F, t26958: F, t26963: F, t26969: F, t26976: F, t26994: F, t26999: F, t27020: F, t27025: F, t27028: F, t29194: F, t29200: F, t3551: F, t3568: F, t3585: F, t3738: F, t3791: F, t460: F, t494: F, t5465: F, t5480: F, t7627: F, t7632: F, t7636: F, t7637: F, t7638: F, t7643: F, t7652: F, t7662: F, t97299: F) -> F {
    let t97453 = t26983 * t7658;
    let t97458 = t97010 * t3153;
    let t97475 = t12627 * t7635;
    let t97480 = -F::new(0.19756347548806534796e1) * t26999 * t3585 - F::new(0.19756347548806534796e1) * t27020 * t3791 + F::new(0.65854491829355115987e0) * t460 * t97299 * t225 * t494 - F::new(0.15612530738769359031e2) * t7636 * t26969 * t7638 * t3738 - F::new(0.26020884564615598386e1) * t7636 * t7637 * t7627 * t3551 + F::new(0.10408353825846239354e2) * t26994 * t7637 * t27028 * t1203 - F::new(0.8673628188205199462e0) * t7636 * t7637 * t2142 * t12689 - F::new(0.26020884564615598386e1) * t97453 * t7662 - F::new(0.26020884564615598386e1) * t27025 * t26963 - F::new(0.26020884564615598386e1) * t29194 * t97458 * t5465 + F::new(0.13010442282307799193e1) * t29200 * t97458 * t5480 - F::new(0.39512695097613069591e1) * t7632 * t13184 - F::new(0.52041769129231196772e1) * t7643 * t7652 * t26958 * t1294 - F::new(0.39512695097613069591e1) * t26976 * t12600 - F::new(0.52041769129231196772e1) * t27025 * t26941 - F::new(0.15612530738769359031e2) * t97475 * t7637 * t7638 * t3568;
    t97480
}
