//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 812/884 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk812<F: Float>(t1843: F, t32261: F, t7064: F, t2558: F, t33360: F, t9647: F, t13194: F, t1841: F, t13168: F, t270: F, t43028: F, t43032: F, t43035: F, t43040: F, t43043: F, t43046: F, t43049: F, t43051: F, t43053: F, t43054: F, t43055: F, t43082: F, t43087: F, t650: F, t681: F, t738: F) -> (F,) {
    let t43090 = t7064 * t1843 * t32261;
    let t43093 = t9647 * t33360 * t2558;
    let t43094 = 0.64087718584518535698e-3 * t43093;
    let t43095 = t1841 * t13194;
    let t43096 = 0.17090058289204942852e-2 * t43095;
    let t43097 = t43028 + t43032 - 0.17090058289204942852e-2 * t43035 - t43040 + t43043 + 0.51270174867614828558e-2 * t43046 - t43049 - 0.46143157380853345702e-1 * t43051 - t43053 + t43054 - t43055 - 0.10254034973522965712e-1 * t650 * t13168 - 0.76905262301422242837e-2 * t681 * t13168 - 0.76905262301422242837e-2 * t270 * t738 * t43082 + 0.15381052460284448567e-1 * t43087 + 0.64087718584518535698e-3 * t43090 + t43094 - t43096;
    (t43097,)
}
