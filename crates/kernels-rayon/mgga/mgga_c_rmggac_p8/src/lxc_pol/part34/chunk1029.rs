//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 1029/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk1029(t76151: f64, t76154: f64, t76159: f64, t71863: f64, t71871: f64, t71892: f64, t76173: f64, t76161: f64, t76163: f64, t76165: f64, t76167: f64, t76169: f64, t76171: f64) -> f64 {
    let t77848 = 0.40911992481368012595e-1_f64 * t76151;
    let t77849 = 0.5454932330849068346e-1_f64 * t76154;
    let t77850 = 0.40911992481368012595e-1_f64 * t76159;
    let t77851 = 0.18183107769496894486e-1_f64 * t71863;
    let t77852 = 0.36366215538993788972e-1_f64 * t71871;
    let t77853 = 0.27274661654245341729e-1_f64 * t71892;
    let t77860 = 0.20455996240684006296e-1_f64 * t76173;
    let t77861 = -t77848 + t77849 + t77850 + t77851 + t77852 - t77853 - 0.18637685463734316849e-1_f64 * t76161 + 0.46594213659335792122e-1_f64 * t76163 + 0.93188427318671584245e-2_f64 * t76165 + 0.46594213659335792124e-1_f64 * t76167 - 0.93188427318671584248e-1_f64 * t76169 - 0.15531404553111930708e-1_f64 * t76171 - t77860;
    t77861
}
