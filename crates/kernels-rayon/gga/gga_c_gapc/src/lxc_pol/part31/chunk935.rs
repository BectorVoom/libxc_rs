//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 935/1447 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk935(t9135: f64, t9138: f64, t9140: f64, t9142: f64, t9145: f64, t9148: f64, t9151: f64, t9153: f64, t9156: f64, t9158: f64, t9161: f64, t9163: f64, t9168: f64) -> f64 {
    let t10722 = 0.27801896084645508334e-2_f64 * t9135 + 0.20241536458333333335e-4_f64 * t9138 - 0.12974218172834570556e-1_f64 * t9140 + 0.2748593934505475288e-5_f64 * t9142 - 0.77294542590142724634e-6_f64 * t9145 + 0.1374296967252737644e-5_f64 * t9148 - 0.42270452978984302532e-6_f64 * t9151 + 0.84540905957968605064e-6_f64 * t9153 + 0.55603792169291016668e-2_f64 * t9156 + 0.67632724766374884052e-4_f64 * t9158 + 0.86956360413910565208e-5_f64 * t9161 - 0.3935854311342592593e-4_f64 * t9163 + 0.76020809606452561851e-3_f64 * t9168;
    t10722
}
