//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 957/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk957(t23575: f64, t3459: f64, t13229: f64, t747: f64, t11135: f64, t7324: f64, t10802: f64, t23555: f64, t13166: f64, t1960: f64, t331: f64, t42470: f64, t42473: f64, t42475: f64, t42478: f64, t42481: f64, t42483: f64, t42485: f64, t42487: f64, t42491: f64, t42494: f64, t42496: f64, t42499: f64, t42979: f64, t43024: f64, t43097: f64, t43149: f64, t43198: f64, t43238: f64, t43287: f64, t43340: f64, t841: f64) -> f64 {
    let t43346 = 4.0_f64 * t23575 * t3459;
    let t43350 = t13229 * t747;
    let t43353 = 4.0_f64 * t7324 * t11135;
    let t43355 = 12.0_f64 * t23555 * t10802;
    let t43356 = (t42979 + t43024 + t43097 + t43149 + t43198 + t43238 + t43287 + t43340) * t331 + t42470 + t42473 + t43346 - t42475 - t42478 + 2.0_f64 * t1960 * t13166 * t841 + t42481 - t43350 * t841 - t42483 + t42485 - t42487 - t42491 - t42494 - t42496 + t43353 - t43355 + t42499;
    t43356
}
