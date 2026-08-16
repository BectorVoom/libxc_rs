//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2155/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2155(t25304: f64, t27212: f64, t25301: f64, t93371: f64, t98857: f64, t27286: f64, t689: f64, t25431: f64, t25411: f64, t27349: f64, t92843: f64, t25314: f64, t25322: f64, t25383: f64, t27183: f64, t27267: f64, t2771: f64, t4534: f64, t7067: f64, t7070: f64, t7766: f64, t7769: f64, t92891: f64, t92895: f64, t92901: f64, t92905: f64, t93118: f64) -> (f64, f64) {
    let t98867 = t25304 * t27212;
    let t98868 = t98867 * t25301;
    let t98875 = t93371 * t98857;
    let t98877 = t27286 * t689;
    let t98879 = 0.14456046980341999104e-1_f64 * t25431 * t98877;
    let t98881 = 0.25702851531048074406e-1_f64 * t25411 * t98877;
    let t98892 = t27349 * t689;
    let t98894 = 0.28912093960683998208e-1_f64 * t92843 * t98892;
    let t98895 = -0.12851425765524037203e-1_f64 * t92891 - 0.22849835011101738147e-2_f64 * t98868 - 0.4336814094102599731e0_f64 * t7766 * t25314 + 0.34270468708064099208e-2_f64 * t92895 - 0.54878743191129263322e-2_f64 * t92901 + 0.96373646535613327358e-3_f64 * t92905 + 0.22849835011101738147e-2_f64 * t98875 - t98879 + t98881 - 0.8673628188205199462e0_f64 * t7067 * t27267 + 0.17347256376410398924e1_f64 * t25383 * t27183 + 0.10408353825846239354e2_f64 * t7070 * t93118 * t7769 * t2771 - 0.13170898365871023197e1_f64 * t25322 * t4534 + t98894;
    (t98892, t98895)
}
