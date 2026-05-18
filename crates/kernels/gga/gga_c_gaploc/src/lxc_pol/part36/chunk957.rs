//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 957/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk957<F: Float>(t23575: F, t3459: F, t13229: F, t747: F, t11135: F, t7324: F, t10802: F, t23555: F, t13166: F, t1960: F, t331: F, t42470: F, t42473: F, t42475: F, t42478: F, t42481: F, t42483: F, t42485: F, t42487: F, t42491: F, t42494: F, t42496: F, t42499: F, t42979: F, t43024: F, t43097: F, t43149: F, t43198: F, t43238: F, t43287: F, t43340: F, t841: F) -> F {
    let t43346 = F::new(4.0) * t23575 * t3459;
    let t43350 = t13229 * t747;
    let t43353 = F::new(4.0) * t7324 * t11135;
    let t43355 = F::new(12.0) * t23555 * t10802;
    let t43356 = (t42979 + t43024 + t43097 + t43149 + t43198 + t43238 + t43287 + t43340) * t331 + t42470 + t42473 + t43346 - t42475 - t42478 + F::new(2.0) * t1960 * t13166 * t841 + t42481 - t43350 * t841 - t42483 + t42485 - t42487 - t42491 - t42494 - t42496 + t43353 - t43355 + t42499;
    t43356
}
