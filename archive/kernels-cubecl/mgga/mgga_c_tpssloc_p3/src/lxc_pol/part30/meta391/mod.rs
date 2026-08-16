//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta391 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1490;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1491;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1492;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1493;
use chunk4::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1494;
use chunk5::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1495;
use chunk6::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1496;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta391<F: Float>(t17178: F, t2768: F, t123: F, t2775: F, t5398: F, t607: F, t882: F, t16558: F, t883: F, t10556: F, t10608: F, t13598: F, t14352: F, t14353: F, t14354: F, t17149: F, t17154: F, t17159: F, t17163: F, t17165: F, t17169: F, t17173: F, t17175: F, t324: F, t300: F, t5689: F, t892: F, t914: F, t11094: F, t5950: F, t3216: F, t5946: F, t4483: F, t4498: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let t17180 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1490::<F>(t17178, t2768, t123);
        let t17183 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1491::<F>(t2775, t5398, t607);
        let t17185 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1492::<F>(t17183, t882, t123);
        let t17187 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1493::<F>(t16558, t883);
        let t17189 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1494::<F>(t17187, t882, t123);
        let t17191 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1495::<F>(t10556, t10608, t13598, t14352, t14353, t14354, t17149, t17154, t17159, t17163, t17165, t17169, t17173, t17175, t17180, t17185, t17189);
        let (t17192, t17194, t17197, t17198, t17202, t17209) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1496::<F>(t17191, t324, t300, t5689, t892, t914, t11094, t5950, t3216, t5946, t4483, t4498);
    (t17180, t17183, t17185, t17187, t17189, t17192, t17194, t17197, t17198, t17202, t17209)
}
