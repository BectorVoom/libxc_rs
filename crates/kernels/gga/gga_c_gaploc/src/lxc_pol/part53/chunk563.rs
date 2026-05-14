//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 563/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk563<F: Float>(t10820: F, t7573: F, t7427: F, t2013: F, t3489: F, t123: F, t2925: F, t883: F, t969: F, t825: F, t2365: F, t8756: F, t7390: F, t3488: F, t7354: F, t2684: F) -> (F, F, F, F, F, F) {
    let t10821 = t7573 * t10820;
    let t10823 = 0.62115540045351614476e2 * t7427 * t10821;
    let t10824 = t2013 * t3489;
    let t10825 = 0.19171462976960374838e0 * t10824;
    let t10826 = t2925 * t123;
    let t10827 = t10826 * t883;
    let t10828 = t969 * t10827;
    let t10829 = t825 * t10828;
    let t10830 = 0.19171462976960374838e0 * t10829;
    let t10834 = t2365 * t8756;
    let t10835 = t7390 * t10834;
    let t10836 = 0.14896037479937677779e-1 * t10835;
    let t10837 = t7354 * t3488;
    let t10838 = t2684 * t10837;
    (t10823, t10825, t10827, t10830, t10836, t10838)
}
