//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1088/1274 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk1088<F: Float>(t14860: F, t231: F, t2662: F, t2661: F, t10722: F, t1565: F, t4352: F, t4366: F, t10726: F, t2430: F, t2747: F, t4365: F, t10762: F, t10783: F, t10812: F, t10816: F, t10900: F, t14843: F, t14846: F, t14850: F, t14853: F, t14859: F, t2745: F, t851: F) -> (F,) {
    let t14861 = t14860 * t231;
    let t14862 = t2662 * t14861;
    let t14864 = 0.14291339372689912324e-4 * t2661 * t14862;
    let t14866 = t10722 * t1565;
    let t14868 = t4352 * t4366;
    let t14869 = t10726 * t14868;
    let t14871 = 0.28582678745379824648e-4 * t2661 * t14869;
    let t14872 = t231 * t2430;
    let t14874 = t2747 * t4365 * t14872;
    let t14878 = -t10900 * t14843 / 4.0 - 0.30488190661738479625e-3 * t14846 - 0.90357964994909313582e-5 * t10762 + 0.10164000561857065645e-3 * t10783 - 0.76220476654346199061e-4 * t14850 - 0.85748036236139473944e-3 * t851 * t14853 - t14859 + t14864 - 0.80031500487063509016e-2 * t10812 - 0.22675591804667994221e-1 * t14866 - t14871 + 0.85748036236139473944e-3 * t2745 * t14874 - 0.11337795902333997111e-1 * t10816;
    (t14878,)
}
