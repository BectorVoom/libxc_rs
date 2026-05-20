//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2155/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2155<F: Float>(t25304: F, t27212: F, t25301: F, t93371: F, t98857: F, t27286: F, t689: F, t25431: F, t25411: F, t27349: F, t92843: F, t25314: F, t25322: F, t25383: F, t27183: F, t27267: F, t2771: F, t4534: F, t7067: F, t7070: F, t7766: F, t7769: F, t92891: F, t92895: F, t92901: F, t92905: F, t93118: F) -> (F, F) {
    let t98867 = t25304 * t27212;
    let t98868 = t98867 * t25301;
    let t98875 = t93371 * t98857;
    let t98877 = t27286 * t689;
    let t98879 = F::cast_from(0.14456046980341999104e-1_f64) * t25431 * t98877;
    let t98881 = F::cast_from(0.25702851531048074406e-1_f64) * t25411 * t98877;
    let t98892 = t27349 * t689;
    let t98894 = F::cast_from(0.28912093960683998208e-1_f64) * t92843 * t98892;
    let t98895 = -F::cast_from(0.12851425765524037203e-1_f64) * t92891 - F::cast_from(0.22849835011101738147e-2_f64) * t98868 - F::cast_from(0.4336814094102599731e0_f64) * t7766 * t25314 + F::cast_from(0.34270468708064099208e-2_f64) * t92895 - F::cast_from(0.54878743191129263322e-2_f64) * t92901 + F::cast_from(0.96373646535613327358e-3_f64) * t92905 + F::cast_from(0.22849835011101738147e-2_f64) * t98875 - t98879 + t98881 - F::cast_from(0.8673628188205199462e0_f64) * t7067 * t27267 + F::cast_from(0.17347256376410398924e1_f64) * t25383 * t27183 + F::cast_from(0.10408353825846239354e2_f64) * t7070 * t93118 * t7769 * t2771 - F::cast_from(0.13170898365871023197e1_f64) * t25322 * t4534 + t98894;
    (t98892, t98895)
}
