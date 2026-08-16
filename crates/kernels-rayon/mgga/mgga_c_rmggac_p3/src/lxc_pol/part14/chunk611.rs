//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 611/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk611(t333: f64, t664: f64, t352: f64, t305: f64, t326: f64, t5266: f64, t7571: f64, t7672: f64, t7772: f64, t7775: f64, t7830: f64, t7832: f64, t7838: f64, t7842: f64, t7845: f64, t7847: f64, t7849: f64, t7853: f64, t7856: f64, t7859: f64, t7863: f64, t7865: f64, t7867: f64, t7869: f64, t7877: f64, t793: f64, t838: f64) -> f64 {
    let t7879 = t664 * t333;
    let t7880 = t7879 * t352;
    let t7883 = -0.6818665413561335432e-1_f64 * t7830 - 0.72732431077987577943e-1_f64 * t7832 - 0.68186654135613354322e-2_f64 * t7838 - 0.18183107769496894486e-1_f64 * t7842 - 0.20455996240684006296e-1_f64 * t7845 - 0.10227998120342003148e-1_f64 * t7847 + 0.13637330827122670864e-1_f64 * t7849 + 0.34093327067806677161e-2_f64 * t7853 - 0.14967802127329760705e-1_f64 * t7856 + 0.19957069503106347607e-1_f64 * t7859 + 0.11974241701863808564e0_f64 * t793 * t7775 + 0.8980681276397856423e-1_f64 * t7863 - 0.17961362552795712846e0_f64 * t7865 - 0.5987120850931904282e-1_f64 * t7867 + 0.17961362552795712846e0_f64 * t7869 + 0.23948483403727617128e0_f64 * t838 * t7772 - 0.59871208509319042821e-1_f64 * t326 * t7672 + 0.59871208509319042821e-1_f64 * t305 * t7571 + 0.2993560425465952141e-1_f64 * t7877 + 0.23948483403727617128e0_f64 * t5266 * t7880;
    t7883
}
