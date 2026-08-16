//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 931/1427 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk931<F: Float>(t9135: F, t9138: F, t9140: F, t9142: F, t9145: F, t9148: F, t9151: F, t9153: F, t9156: F, t9158: F, t9161: F, t9163: F, t9168: F) -> F {
    let t10722 = F::cast_from(0.27801896084645508334e-2_f64) * t9135 + F::cast_from(0.20241536458333333335e-4_f64) * t9138 - F::cast_from(0.12974218172834570556e-1_f64) * t9140 + F::cast_from(0.2748593934505475288e-5_f64) * t9142 - F::cast_from(0.77294542590142724634e-6_f64) * t9145 + F::cast_from(0.1374296967252737644e-5_f64) * t9148 - F::cast_from(0.42270452978984302532e-6_f64) * t9151 + F::cast_from(0.84540905957968605064e-6_f64) * t9153 + F::cast_from(0.55603792169291016668e-2_f64) * t9156 + F::cast_from(0.67632724766374884052e-4_f64) * t9158 + F::cast_from(0.86956360413910565208e-5_f64) * t9161 - F::cast_from(0.3935854311342592593e-4_f64) * t9163 + F::cast_from(0.76020809606452561851e-3_f64) * t9168;
    t10722
}
