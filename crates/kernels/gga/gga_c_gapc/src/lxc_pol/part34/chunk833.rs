//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 833/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk833<F: Float>(t9135: F, t9138: F, t9140: F, t9142: F, t9145: F, t9148: F, t9151: F, t9153: F, t9156: F, t9158: F, t9161: F, t9163: F, t9168: F, t9174: F, t9177: F, t9182: F, t9186: F, t9188: F, t9190: F, t9192: F, t9195: F, t9198: F, t9201: F, t9205: F, t9207: F, t9210: F) -> (F, F) {
    let t10722 = 0.27801896084645508334e-2 * t9135 + 0.20241536458333333335e-4 * t9138 - 0.12974218172834570556e-1 * t9140 + 0.2748593934505475288e-5 * t9142 - 0.77294542590142724634e-6 * t9145 + 0.1374296967252737644e-5 * t9148 - 0.42270452978984302532e-6 * t9151 + 0.84540905957968605064e-6 * t9153 + 0.55603792169291016668e-2 * t9156 + 0.67632724766374884052e-4 * t9158 + 0.86956360413910565208e-5 * t9161 - 0.3935854311342592593e-4 * t9163 + 0.76020809606452561851e-3 * t9168;
    let t10738 = -0.13900948042322754167e-3 * t9174 - 0.13900948042322754167e-3 * t9177 + 0.20011499994481700553e-8 * t9182 + 0.41702844126968262501e-3 * t9186 - 0.4637672555408563478e-4 * t9188 + 0.9275345110817126956e-4 * t9190 + 0.4637672555408563478e-4 * t9192 + 0.77294542590142724634e-6 * t9195 - 0.1374296967252737644e-5 * t9198 + 0.17376185052903442709e-3 * t9201 - 0.28960308421505737848e-5 * t9205 + 0.34752370105806885418e-3 * t9207 + 0.34752370105806885418e-3 * t9210;
    (t10722, t10738)
}
