//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 577/952 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk577<F: Float>(t5271: F, t7199: F, t5259: F, t7205: F, t3814: F, t7710: F, t5245: F, t645: F, t333: F, t664: F, t352: F, t305: F, t326: F, t5266: F, t7571: F, t7672: F, t7772: F, t7775: F, t7830: F, t7832: F, t7838: F, t7842: F, t7845: F, t7847: F, t7849: F, t7853: F, t7856: F, t7859: F, t7863: F, t793: F, t838: F) -> (F,) {
    let t7865 = t5271 * t7199;
    let t7867 = t5259 * t7205;
    let t7869 = t3814 * t7710;
    let t7877 = t5245 * t645;
    let t7879 = t664 * t333;
    let t7880 = t7879 * t352;
    let t7883 = -0.6818665413561335432e-1 * t7830 - 0.72732431077987577943e-1 * t7832 - 0.68186654135613354322e-2 * t7838 - 0.18183107769496894486e-1 * t7842 - 0.20455996240684006296e-1 * t7845 - 0.10227998120342003148e-1 * t7847 + 0.13637330827122670864e-1 * t7849 + 0.34093327067806677161e-2 * t7853 - 0.14967802127329760705e-1 * t7856 + 0.19957069503106347607e-1 * t7859 + 0.11974241701863808564e0 * t793 * t7775 + 0.8980681276397856423e-1 * t7863 - 0.17961362552795712846e0 * t7865 - 0.5987120850931904282e-1 * t7867 + 0.17961362552795712846e0 * t7869 + 0.23948483403727617128e0 * t838 * t7772 - 0.59871208509319042821e-1 * t326 * t7672 + 0.59871208509319042821e-1 * t305 * t7571 + 0.2993560425465952141e-1 * t7877 + 0.23948483403727617128e0 * t5266 * t7880;
    (t7883,)
}
