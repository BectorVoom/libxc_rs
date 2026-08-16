//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta361 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1411;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1412;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta361<F: Float>(t3403: F, t4857: F, t15026: F, t3623: F, t1706: F, t3428: F, t135: F, t457: F, t4936: F, t1174: F, t3431: F, t4912: F, t11583: F, t3961: F, t11529: F, t1709: F, t3432: F, t4889: F, t3450: F, t3966: F, t3448: F, t4928: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t15218, t15245, t15265, t15284, t15285) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1411::<F>(t3403, t4857, t15026, t3623, t1706, t3428, t135, t457, t4936, t1174, t3431, t4912);
        let (t15287, t15293, t15300, t15307, t15313, t15320) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1412::<F>(t1174, t15285, t11583, t3961, t11529, t1709, t3432, t4889, t3450, t3966, t3448, t4928);
    (t15218, t15245, t15265, t15284, t15287, t15293, t15300, t15307, t15313, t15320)
}
