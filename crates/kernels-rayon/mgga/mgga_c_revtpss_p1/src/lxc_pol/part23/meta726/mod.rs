//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta726 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2492;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2493;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta726(t49186: f64, t10142: f64, t14113: f64, t49180: f64, t10136: f64, t14239: f64, t4101: f64, t5740: f64, t9288: f64, t40270: f64, t5737: f64, t10073: f64, t14207: f64, t1398: f64, t14141: f64, t14143: f64, t2434: f64, t14155: f64, t1432: f64, t2470: f64, t3999: f64, t5710: f64, t10069: f64, t14225: f64, t14114: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t49187, t49190, t49199, t49203, t49210, t49238) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2492(t49186, t10142, t14113, t49180, t10136, t14239, t4101, t5740, t9288, t40270, t5737, t10073, t14207);
        let (t49256, t49274, t49276, t49290, t49321) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2493(t1398, t14141, t14143, t2434, t14155, t1432, t2470, t3999, t5710, t10069, t14225, t10136, t14114);
    (t49187, t49190, t49199, t49203, t49210, t49238, t49256, t49274, t49276, t49290, t49321)
}
