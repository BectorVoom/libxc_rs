//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1339/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1339<F: Float>(t26054: F, t9686: F, t25877: F, t94801: F, t25881: F, t1419: F, t786: F, t2022: F, t25909: F, t25921: F, t25926: F, t25930: F, t25931: F, t543: F, t7295: F, t7301: F, t7308: F, t94851: F, t94854: F, t94857: F, t94865: F, t94867: F, t94868: F, t94876: F, t94880: F, t94882: F, t9890: F) -> (F, F) {
    let t94884 = t26054 * t9686;
    let t94886 = t94801 * t25877;
    let t94887 = t94886 * t25881;
    let t94889 = t786 * t1419;
    let t94890 = t94889 * t25877;
    let t94891 = t94890 * t25881;
    let t94893 = F::cast_from(0.72280234901709995519e-3_f64) * t94851 + t94854 + F::cast_from(0.14456046980341999104e-2_f64) * t94857 + F::cast_from(0.4336814094102599731e0_f64) * t7295 * t7301 * t2022 * t9890 * t543 - t94865 - t94867 - F::cast_from(0.26020884564615598386e1_f64) * t25930 * t25931 * t94868 - F::cast_from(0.78062653693846795158e1_f64) * t25921 * t25926 - F::cast_from(0.13010442282307799193e1_f64) * t25909 * t7308 - F::cast_from(0.68549505033305214441e-2_f64) * t94876 - F::cast_from(0.38554277296572111609e-1_f64) * t94880 - F::cast_from(0.38554277296572111609e-1_f64) * t94882 + F::cast_from(0.39029762157531132076e-1_f64) * t94884 + F::cast_from(0.15421710918628844643e0_f64) * t94887 - F::cast_from(0.86736281882051994623e-1_f64) * t94891;
    (t94889, t94893)
}
