//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 680/1120 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk680<F: Float>(t5771: F, t621: F, t5790: F, t226: F, t5317: F, t1966: F, t2009: F, t5589: F, t5748: F, t5754: F, t5755: F, t5761: F, t5766: F, t5770: F, t5774: F, t5777: F, t5781: F, t5782: F, t5785: F, t5787: F, t664: F, t674: F, t682: F, t687: F, t705: F) -> (F, F) {
    let t5791 = t5771 * t621;
    let t5793 = 18.0 * t5790 * t5791;
    let t5794 = t226 * t5317;
    let t5797 = -0.24828486201251232145e5 * t5748 * t2009 * t5589 - t5754 + 0.96491876992155210402e2 * t687 * t5755 * t664 + t5761 + t5766 + t5770 - t5774 - t5777 - 6.0 * t674 * t682 * t1966 - 0.57895126195293126243e3 * t5781 * t5782 + 0.6207121550312808036e4 * t5785 * t5787 - t5793 - 0.11696447245269292414e1 * t705 * t5794;
    (t5793, t5797)
}
