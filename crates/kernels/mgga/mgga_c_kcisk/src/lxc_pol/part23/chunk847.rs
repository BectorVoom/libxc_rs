//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 847/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk847<F: Float>(t3521: F, t3546: F, t1430: F, t3517: F, t1435: F, t1202: F, t3721: F, t333: F, t3724: F, t1171: F, t3631: F, t1167: F, t3676: F, t317: F, t3675: F, t305: F) -> (F, F, F, F, F, F, F, F) {
    let t12857 = t3521 * t3546;
    let t12878 = t3517 * t1430;
    let t12880 = t3517 * t1435;
    let t12884 = 1.0 / t3721 / t1202;
    let t12888 = 1.0 / t3724 / t333;
    let t12900 = t3631 * t1171;
    let t12905 = t1167 * t3676;
    let t12909 = 1.0 / t3675 / t317;
    let t12910 = t305 * t12909;
    (t12857, t12878, t12880, t12884, t12888, t12900, t12905, t12910)
}
