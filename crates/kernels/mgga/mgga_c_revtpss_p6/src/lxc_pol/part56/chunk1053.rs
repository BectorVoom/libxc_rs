//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1053/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1053<F: Float>(t121090: F, t26050: F, t121089: F, t7063: F, t2470: F, t32219: F, t32223: F, t1419: F, t31805: F, t1381: F, t8590: F, t27: F, t3999: F, t8589: F) -> (F, F, F, F, F, F, F) {
    let t121091 = t121090 * t26050;
    let t121093 = t7063 * t121089;
    let t121094 = t121093 * t26050;
    let t121096 = t32219 * t2470;
    let t121098 = F::cast_from(0.34270468708064099208e-1_f64) * t32223 * t121096;
    let t121099 = t31805 * t1419;
    let t121101 = t121099 * t8590 * t1381;
    let t121106 = t8589 * t3999 * t27;
    (t121091, t121093, t121094, t121096, t121098, t121101, t121106)
}
