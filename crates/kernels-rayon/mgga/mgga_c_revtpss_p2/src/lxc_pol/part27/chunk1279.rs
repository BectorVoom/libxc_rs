//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1279/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1279(t26054: f64, t9686: f64, t25877: f64, t94801: f64, t25881: f64, t1419: f64, t786: f64, t2022: f64, t25909: f64, t25921: f64, t25926: f64, t25930: f64, t25931: f64, t543: f64, t7295: f64, t7301: f64, t7308: f64, t94851: f64, t94854: f64, t94857: f64, t94865: f64, t94867: f64, t94868: f64, t94876: f64, t94880: f64, t94882: f64, t9890: f64) -> (f64, f64) {
    let t94884 = t26054 * t9686;
    let t94886 = t94801 * t25877;
    let t94887 = t94886 * t25881;
    let t94889 = t786 * t1419;
    let t94890 = t94889 * t25877;
    let t94891 = t94890 * t25881;
    let t94893 = 0.72280234901709995519e-3_f64 * t94851 + t94854 + 0.14456046980341999104e-2_f64 * t94857 + 0.4336814094102599731e0_f64 * t7295 * t7301 * t2022 * t9890 * t543 - t94865 - t94867 - 0.26020884564615598386e1_f64 * t25930 * t25931 * t94868 - 0.78062653693846795158e1_f64 * t25921 * t25926 - 0.13010442282307799193e1_f64 * t25909 * t7308 - 0.68549505033305214441e-2_f64 * t94876 - 0.38554277296572111609e-1_f64 * t94880 - 0.38554277296572111609e-1_f64 * t94882 + 0.39029762157531132076e-1_f64 * t94884 + 0.15421710918628844643e0_f64 * t94887 - 0.86736281882051994623e-1_f64 * t94891;
    (t94889, t94893)
}
