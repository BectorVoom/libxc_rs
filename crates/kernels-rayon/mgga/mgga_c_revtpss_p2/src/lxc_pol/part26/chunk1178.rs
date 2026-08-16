//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1178/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1178(t93377: f64, t95726: f64, t10495: f64, t10977: f64, t1956: f64, t1957: f64, t2061: f64, t233: f64, t25383: f64, t26489: f64, t7070: f64, t7071: f64, t7403: f64, t95624: f64, t95629: f64, t95632: f64, t95635: f64, t95645: f64, t95647: f64, t95649: f64, t95651: f64, t95715: f64, t95720: f64, t95722: f64) -> f64 {
    let t95727 = t93377 * t95726;
    let t95729 = 0.15421710918628844643e0_f64 * t95624 + 0.39512695097613069591e1_f64 * t7403 * t10495 - 0.10281140612419229762e0_f64 * t95629 + t95632 - 0.16463622957338778996e-1_f64 * t95635 - 0.78062653693846795158e1_f64 * t25383 * t26489 + 0.8673628188205199462e0_f64 * t7070 * t7071 * t2061 * t10977 + 0.21684070470512998656e-1_f64 * t95645 - 0.38554277296572111609e-1_f64 * t95647 + 0.77108554593144223218e-1_f64 * t95649 - 0.29272321618148349057e-1_f64 * t95651 - 0.4336814094102599731e0_f64 * t1956 * t1957 * t233 * t95715 - 0.43368140941025997312e-1_f64 * t95720 + 0.57824187921367996415e-1_f64 * t95722 - 0.10281140612419229763e-1_f64 * t95727;
    t95729
}
