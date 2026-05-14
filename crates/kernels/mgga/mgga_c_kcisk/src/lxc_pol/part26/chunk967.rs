//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 967/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk967<F: Float>(t20302: F, t20406: F, t26130: F, t26141: F, t26144: F, t26147: F, t26153: F, t26171: F, t26173: F, t26176: F, t26179: F, t13748: F, t20298: F, t20438: F, t20440: F, t20454: F, t26204: F, t26207: F, t26209: F, t26212: F, t26215: F, t26217: F) -> (F, F) {
    let t26267 = -0.5519e-1 * t26130 + 0.44152e0 * t20406 + 0.40256666666666666668e0 * t20302 + 0.16504875e0 * t26171 + 0.258925e1 * t26173 + 0.36793333333333333333e-1 * t26176 - 0.22076e0 * t26179 - 0.33547222222222222222e0 * t26141 + 0.12077e1 * t26144 - 0.80513333333333333332e0 * t26147 - 0.181155e1 * t26153;
    let t26287 = -t20438 + 0.73586666666666666667e-1 * t20440 - 0.40256666666666666668e0 * t20298 + t20454 + 0.82524375e-1 * t26204 + 0.19419375e1 * t26207 - 0.258925e1 * t26209 - 0.1294625e1 * t26212 - 0.412621875e-1 * t26215 + 0.16504875e0 * t26217 - t13748;
    (t26267, t26287)
}
