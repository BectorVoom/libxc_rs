//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1201/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1201<F: Float>(t125182: F, t125184: F, t132135: F, t132141: F, t132144: F, t132152: F, t132167: F, t1456: F, t1458: F, t1914: F, t2168: F, t29490: F, t33572: F, t35034: F, t5790: F, t7691: F, t7700: F, t8241: F, t8249: F, t8978: F) -> F {
    let t132170 = F::new(2.0) * t8241 * t7700 + F::new(2.0) * t132135 + F::new(2.0) * t7691 * t8249 + F::new(2.0) * t2168 * t29490 + t125182 + t125184 + t132141 + t5790 * t8978 + t1914 * t33572 + t132144 + t1456 * t35034 + t1458 * (t132152 + t132167);
    t132170
}
