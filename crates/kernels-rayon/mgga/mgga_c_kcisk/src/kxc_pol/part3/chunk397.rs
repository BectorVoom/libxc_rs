//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 397/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk397(t2864: f64, t2867: f64, t2869: f64, t2873: f64, t2875: f64, t2877: f64, t830: f64, t815: f64, t813: f64, t14: f64, t31: f64, t2857: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2879 = -0.42198333333333333333e0_f64 * t2864 + 0.84396666666666666666e0_f64 * t2867 + 0.39862222222222222223e0_f64 * t2869 + 0.68258333333333333333e-1_f64 * t2873 + 0.13651666666666666667e0_f64 * t2875 + 0.13692777777777777778e0_f64 * t2877;
    let t2880 = t2879 * t830;
    let t2882 = 1.0_f64 * t815 * t2880;
    let t2883 = t813 * t813;
    let t2884 = 1.0_f64 / t2883;
    let t2885 = t14 * t2884;
    let t2886 = t31 * t31;
    let t2887 = 1.0_f64 / t2886;
    let t2888 = t2857 * t2887;
    (t2879, t2880, t2882, t2883, t2884, t2885, t2886, t2887, t2888)
}
