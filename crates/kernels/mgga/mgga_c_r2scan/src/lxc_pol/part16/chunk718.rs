//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 718/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk718<F: Float>(t1966: F, t2009: F, t5589: F, t5748: F, t5754: F, t5755: F, t5761: F, t5766: F, t5770: F, t5774: F, t5777: F, t5781: F, t5782: F, t5785: F, t5787: F, t5793: F, t5794: F, t664: F, t674: F, t682: F, t687: F, t705: F) -> F {
    let t5797 = -F::cast_from(0.24828486201251232145e5_f64) * t5748 * t2009 * t5589 - t5754 + F::cast_from(0.96491876992155210402e2_f64) * t687 * t5755 * t664 + t5761 + t5766 + t5770 - t5774 - t5777 - F::cast_from(6.0_f64) * t674 * t682 * t1966 - F::cast_from(0.57895126195293126243e3_f64) * t5781 * t5782 + F::cast_from(0.6207121550312808036e4_f64) * t5785 * t5787 - t5793 - F::cast_from(0.11696447245269292414e1_f64) * t705 * t5794;
    t5797
}
