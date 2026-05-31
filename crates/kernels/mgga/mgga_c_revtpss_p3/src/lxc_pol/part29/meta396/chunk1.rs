//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1418/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1418<F: Float>(t14869: F, t2661: F, t231: F, t2430: F, t2747: F, t4365: F, t10762: F, t10783: F, t10812: F, t10816: F, t10900: F, t14843: F, t14846: F, t14850: F, t14853: F, t14859: F, t14864: F, t14866: F, t2745: F, t851: F) -> (F, F) {
    let t14871 = F::cast_from(0.28582678745379824648e-4_f64) * t2661 * t14869;
    let t14872 = t231 * t2430;
    let t14874 = t2747 * t4365 * t14872;
    let t14878 = -t10900 * t14843 / F::cast_from(4.0_f64) - F::cast_from(0.30488190661738479625e-3_f64) * t14846 - F::cast_from(0.90357964994909313582e-5_f64) * t10762 + F::cast_from(0.10164000561857065645e-3_f64) * t10783 - F::cast_from(0.76220476654346199061e-4_f64) * t14850 - F::cast_from(0.85748036236139473944e-3_f64) * t851 * t14853 - t14859 + t14864 - F::cast_from(0.80031500487063509016e-2_f64) * t10812 - F::cast_from(0.22675591804667994221e-1_f64) * t14866 - t14871 + F::cast_from(0.85748036236139473944e-3_f64) * t2745 * t14874 - F::cast_from(0.11337795902333997111e-1_f64) * t10816;
    (t14874, t14878)
}
