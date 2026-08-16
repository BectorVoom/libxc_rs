//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta188 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk918;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk919;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk920;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk921;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta188<F: Float>(t1742: F, t372: F, t479: F, t471: F, t1230: F, t248: F, t4733: F, t3440: F, t4724: F, t1193: F, t1706: F, t135: F, t1725: F, t1174: F, t1196: F, t3966: F, t974: F, t1198: F, t1213: F, t1218: F, t1227: F, t1232: F, t1748: F, t3490: F, t3524: F, t3542: F, t3543: F, t3547: F, t3549: F, t3573: F, t4889: F, t5014: F, t5019: F, t5010: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t5022, t5023, t5024) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk918::<F>(t1742, t372, t479, t471);
        let t5030 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk919::<F>(t1230, t248, t4733);
        let (t5033, t5036, t5040, t5041, t5045, t5046, t5051) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk920::<F>(t3440, t4724, t1193, t1706, t135, t1725, t1174, t1196, t3966, t974, t1198, t1213, t1218, t1227, t1232, t1748, t3490, t3524, t3542, t3543, t3547, t3549, t3573, t4889, t5014, t5019, t5024, t5030);
        let t5052 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk921::<F>(t5010, t5051);
    (t5022, t5023, t5024, t5030, t5033, t5036, t5040, t5041, t5045, t5046, t5052)
}
