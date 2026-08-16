//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta547 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1898;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta547<F: Float>(t24633: F, t8002: F, t254: F, t492: F, t11605: F, t2154: F, t5059: F, t225: F, t8055: F, t2123: F, t4930: F, t1238: F, t1252: F, t14972: F, t15820: F, t1761: F, t2121: F, t2155: F, t24646: F, t24893: F, t27549: F, t27761: F, t27767: F, t27770: F, t27776: F, t3593: F, t4945: F, t5060: F, t7283: F, t7351: F, t7356: F, t8088: F) -> (F, F, F, F, F, F, F) {
        let (t27779, t27784, t27785, t27786, t27792, t27794, t27797) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1898::<F>(t24633, t8002, t254, t492, t11605, t2154, t5059, t225, t8055, t2123, t4930, t1238, t1252, t14972, t15820, t1761, t2121, t2155, t24646, t24893, t27549, t27761, t27767, t27770, t27776, t3593, t4945, t5060, t7283, t7351, t7356, t8088);
    (t27779, t27784, t27785, t27786, t27792, t27794, t27797)
}
